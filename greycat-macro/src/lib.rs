use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro]
pub fn gc_type_id(input: TokenStream) -> TokenStream {
    let ident = syn::parse_macro_input!(input as syn::Ident);
    let static_ident = format_ident!("_{ident}");

    let expanded = quote! {
      static mut #static_ident: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
      #[inline(always)]
      pub(crate) fn #ident() -> ::greycat::GcTypeId {
        unsafe { #static_ident }
      }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn greycat_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as syn::ItemStruct);
    let vis = &item_struct.vis;
    let struct_ident = &item_struct.ident;
    let generics = &item_struct.generics;
    let fields = item_struct.fields.iter();
    let finalizer_const = format_ident!("_gc_{struct_ident}_finalize");
    let finalizer_fn = format_ident!("__gc_{struct_ident}_finalize_fn");

    if let syn::Fields::Unnamed(_) = item_struct.fields {
        return TokenStream::from(
            syn::Error::new(
                item_struct.fields.span(),
                "#[greycat] only supports struct with named fields or empty struct",
            )
            .to_compile_error(),
        );
    }

    let expanded = quote! {
        #[repr(C)]
        #vis struct #struct_ident #generics {
          pub(crate) __header: ::greycat::GcObjectOwned,
          #(#fields),*
        }

        impl ::greycat::object::AsPtr for CsvReader {
            fn as_ptr(&self) -> *const ::greycat::sys::gc_object_t {
                self as *const Self as *const _
            }
        }

        impl ::greycat::object::AsPtrMut for CsvReader {
            fn as_ptr_mut(&mut self) -> *mut ::greycat::sys::gc_object_t {
                self as *mut Self as *mut _
            }
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn #finalizer_fn(
            this: *mut ::greycat::sys::gc_object_t,
            ctx: *mut ::greycat::sys::gc_machine_t,
        ) {
            let ctx = ::greycat::GcMachine(ctx);
            let this = unsafe { &mut *(this as *mut #struct_ident) };
            #struct_ident::finalize(this, ctx);
        }
        #[allow(non_upper_case_globals)]
        pub(crate) static #finalizer_const: Option<::greycat::GcObjectFinalizeFn> = Some(#finalizer_fn);
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

    for item in &item_impl.items {
        match item {
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

    let expanded = quote! {
      #item_impl

      #(#bindings)*
    };

    TokenStream::from(expanded)
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

#[proc_macro_derive(Object)]
pub fn derive_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let expanded = quote! {
      impl ::greycat::Object for #name {}
    };
    TokenStream::from(expanded)
}
