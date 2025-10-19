use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn greycat_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as syn::ItemStruct);
    let vis = &item_struct.vis;
    let struct_ident = &item_struct.ident;
    let generics = &item_struct.generics;
    let fields = item_struct.fields.iter();

    if let syn::Fields::Unnamed(_) = item_struct.fields {
        return TokenStream::from(
            syn::Error::new(
                item_struct.fields.span(),
                "#[greycat] only supports struct with named fields or empty struct",
            )
            .to_compile_error(),
        );
    }

    // Generate the rewritten struct
    let expanded = quote! {
      #[repr(C)]
      #vis struct #struct_ident #generics {
        pub(crate) __header: ::greycat::GcObject,
        #(#fields)*
      }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn greycat_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(item as syn::ItemImpl);

    let type_name = if let syn::Type::Path(p) = &*item_impl.self_ty {
        p.path.segments.last().unwrap().ident.clone()
    } else {
        return TokenStream::from(
            syn::Error::new(
                item_impl.self_ty.span(),
                "unsupported type, expected path type",
            )
            .to_compile_error(),
        );
    };

    let mut bindings = Vec::new();
    let mut has_finalizer = false;

    for item in &item_impl.items {
        match item {
            syn::ImplItem::Fn(m) if is_finalizer(m) => {
                if has_finalizer {
                    return TokenStream::from(
                        syn::Error::new(m.span(), "only one method can be a #[finalize]")
                            .to_compile_error(),
                    );
                }

                let fn_ident = &m.sig.ident;
                let binding_name = format_ident!("_gc_{type_name}_finalize");
                let finalizer = format_ident!("__gc_{type_name}_finalize_fn");

                bindings.push(quote! {
                  #[allow(non_snake_case)]
                  unsafe extern "C" fn #finalizer(
                    this: *mut ::greycat::sys::gc_object_t,
                    ctx: *mut ::greycat::sys::gc_machine_t,
                  ) {
                    let ctx = ::greycat::GcMachine(ctx);
                    let this = unsafe { &mut *(this as *mut #type_name) };
                    #type_name::#fn_ident(this, ctx);
                  }
                  #[allow(non_upper_case_globals)]
                  pub(crate) static #binding_name: Option<::greycat::GcObjectFinalizeFn> = Some(#finalizer);
                });
                has_finalizer = true;
            }
            syn::ImplItem::Fn(m) => {
                let fn_ident = &m.sig.ident;
                let binding_name = format_ident!("_gc_{type_name}__{fn_ident}");

                // detect if the return type is a Result
                let set_result = match &m.sig.output {
                    syn::ReturnType::Type(_, ty) => match &**ty {
                        syn::Type::Path(tp)
                            if matches!(
                                &*tp.path.segments.last().unwrap().ident.to_string(),
                                "Result" | "GcResult"
                            ) =>
                        {
                            quote! {
                              match #type_name::#fn_ident(this, ctx) {
                                Ok(value) => ctx.set_result(value),
                                Err(err) => ctx.set_error(err),
                              }
                            }
                        }
                        _ => quote! {
                          ctx.set_result(#type_name::#fn_ident(this, ctx));
                        },
                    },
                    syn::ReturnType::Default => quote! {},
                };

                bindings.push(quote! {
                  #[allow(non_snake_case)]
                  pub(crate) unsafe extern "C" fn #binding_name(ctx: *mut ::greycat::sys::gc_machine_t) {
                      let ctx = ::greycat::GcMachine(ctx);
                      let this = unsafe { ctx.get_self_mut::<#type_name>() };
                      #set_result
                  }
                });
            }
            _ => todo!(),
        }
    }

    if !has_finalizer {
        let binding_name = format_ident!("_gc_{type_name}_finalize");
        bindings.push(quote! {
          #[allow(non_upper_case_globals)]
          pub(crate) static #binding_name: Option<::greycat::GcObjectFinalizeFn> = None;
        });
    }

    let expanded = quote! {
      #item_impl

      #(#bindings)*
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn finalize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn greycat_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_ident = &item_fn.sig.ident;
    let binding_name = format_ident!("_gc_{fn_ident}");

    let mut extract_args = Vec::new();
    let mut args = Vec::new();

    for (i, input) in item_fn.sig.inputs.iter().enumerate() {
        match input {
            syn::FnArg::Typed(pat_ty) => {
                match &*pat_ty.ty {
                    syn::Type::Path(tp)
                        if tp.path.segments.last().unwrap().ident == "GcMachine" =>
                    {
                        // machine argument
                        args.push(quote! { ctx });
                    }
                    _ => {
                        // other argument
                        let pat = &*pat_ty.pat;
                        let idx = syn::Index::from(i);
                        extract_args.push(quote! {
                          let #pat = unsafe { ctx.get_param(#idx) };
                        });
                        args.push(quote! { #pat });
                    }
                }
            }
            syn::FnArg::Receiver(_) => {
                return syn::Error::new_spanned(input, "self parameter not supported")
                    .to_compile_error()
                    .into();
            }
        }
    }

    // detect if the return type is a Result
    let set_result = match &item_fn.sig.output {
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(tp) if tp.path.segments.last().unwrap().ident == "Result" => quote! {
              match #fn_ident(#(#args),*) {
                Ok(value) => ctx.set_result(value),
                Err(err) => ctx.set_error(err),
              }
            },
            _ => quote! {
              ctx.set_result(#fn_ident(#(#args),*));
            },
        },
        syn::ReturnType::Default => quote! {},
    };

    let expanded = quote! {
      #item_fn

      #[allow(non_snake_case)]
      pub(crate) unsafe extern "C" fn #binding_name(ctx: *mut ::greycat::sys::gc_machine_t) {
          let ctx = ::greycat::GcMachine(ctx);
          #(#extract_args)*
          #set_result
      }
    };

    TokenStream::from(expanded)
}

fn is_finalizer(item_fn: &syn::ImplItemFn) -> bool {
    for attr in &item_fn.attrs {
        if attr.path().is_ident("finalize") {
            return true;
        }
    }
    false
}
