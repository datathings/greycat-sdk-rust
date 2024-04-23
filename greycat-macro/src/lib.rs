use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Type)]
pub fn derive_type(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_greycat_type_trait(&ast)
}

#[proc_macro_attribute]
pub fn greycat_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(_) => {
                let struct_name = ast.ident;
                let attr = ast.attrs.iter();
                quote! {
                    #[derive(Type)]
                    #( #attr )*
                    pub struct #struct_name {
                        obj: ::greycat::GObj,
                        ctx: ::greycat::Machine,
                    }
                }
                .into()
            }
            Fields::Unit => {
                let struct_name = ast.ident;
                let attr = ast.attrs.iter();
                quote! {
                    #[derive(Type)]
                    #( #attr )*
                    pub struct #struct_name {
                        obj: ::greycat::GObj,
                        ctx: ::greycat::Machine,
                    }
                }
                .into()
            }
            _ => panic!("`#[greycat_object]` does not support unnamed struct"),
        },
        _ => panic!("`#[greycat_object]` has to be used on a struct"),
    }
}

fn impl_greycat_type_trait(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    quote! {
        impl #struct_name {
            #[inline(always)]
            pub fn new(ctx: ::greycat::Machine, obj: ::greycat::GObj) -> Self {
                Self { ctx, obj }
            }
        }
    }
    .into()
}

// #[proc_macro_attribute]
// pub fn greycat_bindgen(_args: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as ItemFn);
//     let ItemFn {
//         attrs,
//         vis,
//         sig,
//         block,
//     } = input;
//     let Signature { ident, inputs, .. } = sig;
//     let args = inputs
//         .iter()
//         .filter_map(|arg| match arg {
//             FnArg::Receiver(_) => None,
//             FnArg::Typed(arg) => Some(arg),
//         }).collect::<Vec<_>>();
//     let arg_name = args.iter().map(|arg| &arg.pat);
//     let arg_type = args.iter().map(|arg| &arg.ty);
//     let arg_offset = args.iter().enumerate().map(|(index, _)| index);
//     let stmts = &block.stmts;
//     quote! {
//         #[allow(non_snake_case)]
//         pub unsafe extern "C" fn #ident (ctx: *mut gc_machine_t) {
//             let ctx = Machine::from(ctx);
//             #(let #arg_name = ctx.get_ #arg_type _param(#arg_offset);)*

//             #(#stmts)*
//         }
//     }
//     .into()
// }
