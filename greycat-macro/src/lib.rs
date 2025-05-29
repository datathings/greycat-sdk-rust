use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn greycat(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the original struct
    let input = parse_macro_input!(item as ItemStruct);
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let mut fields = input.fields;

    // Create the new header field
    let header_field: syn::Field = syn::parse_quote! {
      pub(super) header: ::greycat::gc_object_t
    };

    // Only handle named fields
    match &mut fields {
        Fields::Named(ref mut named_fields) => {
            // Insert header field at the beginning
            named_fields.named.insert(0, header_field);
        }
        _ => {
            return syn::Error::new_spanned(
                ident,
                "#[greycat] only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    }

    // Generate the rewritten struct
    let expanded = quote! {
      #[allow(unused)]
      #[repr(C)]
      #vis struct #ident #generics #fields
    };

    TokenStream::from(expanded)
}
