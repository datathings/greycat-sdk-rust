mod parsers;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned};

use crate::parsers::is_finalizer;

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
                    let this = unsafe { ctx.get_self_mut::<#type_name>() };
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
                bindings.push(quote! {
                  #[allow(non_snake_case)]
                  pub(crate) unsafe extern "C" fn #binding_name(ctx: *mut ::greycat::sys::gc_machine_t) {
                      let ctx = ::greycat::GcMachine(ctx);
                      let this = unsafe { ctx.get_self_mut::<#type_name>() };
                      ctx.set_result(#type_name::#fn_ident(this, ctx));
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

// #[proc_macro_attribute]
// pub fn greycat_type(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let GreycatTypeAttr { module } = parse_macro_input!(attr as GreycatTypeAttr);
//     let item_impl = parse_macro_input!(input as syn::ItemImpl);

//     let type_name = if let syn::Type::Path(p) = &*item_impl.self_ty {
//         p.path.segments.last().unwrap().ident.clone()
//     } else {
//         return TokenStream::from(
//             syn::Error::new(
//                 item_impl.self_ty.span(),
//                 "unsupported type, expected path type",
//             )
//             .to_compile_error(),
//         );
//     };

//     let mut fn_bindings = Vec::new();
//     let mut link_calls = Vec::new();
//     // let mut finalize_fn = None;

//     /*
//       for attr in &m.attrs {
//         if attr.path().is_ident("greycat_method") {

//         } else if attr.path().is_ident("greycat_finalize") {
//           // create the finalizer binding
//           let fn_ident = &m.sig.ident;
//           let finalize_binding_name = format_ident!("__gc__{type_name}__{fn_ident}");

//           fn_bindings.push(quote! {
//             unsafe extern "C" fn #finalize_binding_name(this: *mut ::greycat::gc_object_t, ctx: *mut ::greycat::gc_machine_t) {
//               #type_name::#fn_ident(&mut *(this as *mut #type_name), ::greycat::GcMachine::from(ctx));
//             }
//           });

//           finalize_fn = Some(finalize_binding_name);
//         }
//       }

//     */
//     for item in &item_impl.items {
//         match item {
//             syn::ImplItem::Fn(m) => {
//                 // create the fn binding
//                 let rename = match parse_method_attr(&m.attrs) {
//                   Ok(v) => v,
//                   Err(e) => return e,
//                 };
//                 let fn_name = rename.unwrap_or_else(|| m.sig.ident.to_string());
//                 let fn_ident = &m.sig.ident;

//                 let binding_name = format_ident!("__gc__{type_name}__{fn_name}");

//                 fn_bindings.push(quote! {
//                   unsafe extern "C" fn #fn_binding_name(ctx: *mut ::greycat::gc_machine_t) {
//                     let ctx = ::greycat::GcMachine::from(ctx);
//                     let this = &mut *ctx.get_self::<#type_name>();
//                     match #type_name::#fn_ident(this, ctx) {
//                       Ok(value) => ctx.set_result(value),
//                       Err(message) => ctx.set_error(message.to_string()),
//                     }
//                   }
//                 });

//                 link_calls.push(quote! {
//                   #[::linkme::distributed_slice(::greycat::link::TYPE_FNS)]
//                   static #static_name: ::greycat::link::GcTypeFn = ::greycat::link::GcTypeFn {
//                     module_name: #module_name,
//                     fn_name: #fn_name,
//                     ptr: #binding_name,
//                   };
//                 });
//             }
//             _ => todo!(),
//         }
//     }

//     // create register function name and push to distributed slice
//     // let register_ident = format_ident!("gc_lib_{library}__link");

//     // registration function body: resolve module/type, configure_type, link fns
//     // let configure_type = match finalize_fn {
//     //     Some(finalize_fn) => quote! {
//     //       prog.configure_type(type_id, std::mem::size_of::<#type_name>() as u32, #finalize_fn);
//     //     },
//     //     None => quote! {
//     //       prog.configure_type(type_id, ::std::mem::size_of::<#type_name>() as u32, None);
//     //     },
//     // };

//     quote! {
//       #item_impl

//     }
//     .into()
// }

// #[proc_macro_attribute]
// pub fn greycat_fn(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let attrs = parse_macro_input!(attr as FnAttr);
//     let item_fn = parse_macro_input!(input as syn::ItemFn);
//     let fn_ident = &item_fn.sig.ident;
//     let fn_name = attrs
//         .rename
//         .unwrap_or_else(|| item_fn.sig.ident.to_string());
//     let module_name = attrs.module_name;
//     let binding_name = format_ident!("__gc_{module_name}_{fn_name}");
//     let static_name = format_ident!("__gc_{module_name}_{fn_name}_static");

//     // collect argument extraction code
//     let mut extract_stmts = Vec::new();
//     let mut call_args = Vec::new();

//     for (i, input) in item_fn.sig.inputs.iter().enumerate() {
//         match input {
//             syn::FnArg::Typed(pat_ty) => {
//                 let pat = &pat_ty.pat;
//                 let ty = &pat_ty.ty;

//                 // detect if it's Machine
//                 let is_machine = matches!(
//                   **ty,
//                   syn::Type::Path(ref type_path)
//                     if type_path.path.segments.last().unwrap().ident == "Machine"
//                 );

//                 if is_machine {
//                     call_args.push(quote! { ctx });
//                 } else {
//                     let idx = syn::Index::from(i);
//                     extract_stmts.push(quote! {
//                       let #pat = unsafe { ctx.get_param(#idx) };
//                     });
//                     call_args.push(quote! { #pat });
//                 }
//             }
//             syn::FnArg::Receiver(_) => {
//                 return syn::Error::new_spanned(input, "self parameter not supported")
//                     .to_compile_error()
//                     .into();
//             }
//         }
//     }

//     // detect if return type is Result
//     let is_result = match &item_fn.sig.output {
//         syn::ReturnType::Type(_, ty) => match **ty {
//             syn::Type::Path(ref tp) => tp.path.segments.last().unwrap().ident == "Result",
//             _ => false,
//         },
//         syn::ReturnType::Default => false,
//     };

//     let call_expr = if is_result {
//         quote! {
//           match #fn_ident(#(#call_args),*) {
//             Ok(value) => ctx.set_result(value),
//             Err(message) => ctx.set_error(message.to_string()),
//           }
//         }
//     } else {
//         quote! {
//           ctx.set_result(#fn_ident(#(#call_args),*));
//         }
//     };

//     let binding = quote! {
//       #[unsafe(no_mangle)]
//       unsafe extern "C" fn #binding_name(ctx: *mut ::greycat::gc_machine_t) {
//           let ctx = ::greycat::GcMachine::from(ctx);
//           #(#extract_stmts)*
//           #call_expr
//       }
//     };

//     quote! {
//       #item_fn

//       #binding

//       #[::linkme::distributed_slice(::greycat::link::MODULE_FNS)]
//       static #static_name: ::greycat::link::GcModuleFn = ::greycat::link::GcModuleFn {
//         module_name: #module_name,
//         fn_name: #fn_name,
//         ptr: #binding_name,
//       };
//     }
//     .into()
// }

// #[proc_macro_attribute]
// pub fn greycat_method(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let attrs = parse_macro_input!(attr as MethodAttr);
//     let item_fn = parse_macro_input!(input as syn::ImplItemFn);
//     let fn_ident = &item_fn.sig.ident;
//     let fn_name = attrs
//         .rename
//         .unwrap_or_else(|| item_fn.sig.ident.to_string());
//     let module_name = attrs.module_name;
//     let type_name = attrs.type_name;
//     let binding_name = format_ident!("__gc_{module_name}_{type_name}_{fn_name}");
//     let static_name = format_ident!("__gc_{module_name}_{type_name}_{fn_name}_static");

//     // collect argument extraction code
//     let mut extract_stmts = Vec::new();
//     let mut call_args = Vec::new();

//     for (i, input) in item_fn.sig.inputs.iter().enumerate() {
//         match input {
//             syn::FnArg::Typed(pat_ty) => {
//                 let pat = &pat_ty.pat;
//                 let ty = &pat_ty.ty;

//                 // detect if it's Machine
//                 let is_machine = matches!(
//                   **ty,
//                   syn::Type::Path(ref type_path)
//                     if type_path.path.segments.last().unwrap().ident == "Machine"
//                 );

//                 if is_machine {
//                     call_args.push(quote! { ctx });
//                 } else {
//                     let idx = syn::Index::from(i);
//                     extract_stmts.push(quote! {
//                       let #pat = unsafe { ctx.get_param(#idx) };
//                     });
//                     call_args.push(quote! { #pat });
//                 }
//             }
//             syn::FnArg::Receiver(_) => {
//                 return syn::Error::new_spanned(input, "self parameter not supported")
//                     .to_compile_error()
//                     .into();
//             }
//         }
//     }

//     // detect if return type is Result
//     let is_result = match &item_fn.sig.output {
//         syn::ReturnType::Type(_, ty) => match **ty {
//             syn::Type::Path(ref tp) => tp.path.segments.last().unwrap().ident == "Result",
//             _ => false,
//         },
//         syn::ReturnType::Default => false,
//     };

//     let call_expr = if is_result {
//         quote! {
//           match #fn_ident(#(#call_args),*) {
//             Ok(value) => ctx.set_result(value),
//             Err(message) => ctx.set_error(message.to_string()),
//           }
//         }
//     } else {
//         quote! {
//           ctx.set_result(#fn_ident(#(#call_args),*));
//         }
//     };

//     let binding = quote! {
//       #[unsafe(no_mangle)]
//       unsafe extern "C" fn #binding_name(ctx: *mut ::greycat::gc_machine_t) {
//         let ctx = ::greycat::GcMachine::from(ctx);
//         #(#extract_stmts)*
//         #call_expr
//       }
//     };

//     quote! {
//       #item_fn

//       #binding

//       #[::linkme::distributed_slice(::greycat::link::TYPE_FNS)]
//       static #static_name: ::greycat::link::GcTypeFn = ::greycat::link::GcTypeFn {
//         module_name: #module_name,
//         fn_name: #fn_name,
//         ptr: #binding_name,
//       };
//     }
//     .into()
// }

// #[proc_macro_attribute]
// pub fn greycat_finalize(_args: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
