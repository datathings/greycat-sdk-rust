#![allow(unused)]

use syn::ImplItemFn;

pub fn is_finalizer(item_fn: &ImplItemFn) -> bool {
    for attr in &item_fn.attrs {
        if attr.path().is_ident("finalize") {
            return true;
        }
    }
    false
}

pub struct GreycatTypeAttr {
    pub module: String,
}

impl syn::parse::Parse for GreycatTypeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let name: syn::LitStr = input.parse()?;

        if ident == "module" {
            return Ok(Self {
                module: name.value(),
            });
        }
        // if input.is_empty() {
        //   // TODO validate
        //   return Self::validate(ident, name);
        // }

        // input.parse::<syn::Token![,]>()?;

        // let ident_b: syn::Ident = input.parse()?;
        // input.parse::<syn::Token![=]>()?;
        // let lit_b: syn::LitStr = input.parse()?;

        // match (&*ident_a.to_string(), &*ident_b.to_string()) {
        //     ("library", "module") => Ok(Self {
        //         library: lit_a.value(),
        //         module: lit_b.value(),
        //     }),
        //     ("module", "library") => Ok(Self {
        //         library: lit_b.value(),
        //         module: lit_a.value(),
        //     }),
        //     _ => Err(syn::Error::new(
        //         input.span(),
        //         "expected `library = \"...\", module = \"...\"`",
        //     )),
        // }
        Err(syn::Error::new(input.span(), "expected `module = \"...\"`"))
    }
}

#[derive(Default)]
pub struct FnAttr {
    pub module_name: String,
    pub rename: Option<String>,
}

impl syn::parse::Parse for FnAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "missing module name"));
        }

        let lit = input.parse::<syn::LitStr>()?;
        let str = lit.value();
        match str.rsplit_once("::") {
            Some((module, "")) => Ok(Self {
                rename: None,
                module_name: module.to_string(),
            }),
            Some((module, rename)) => Ok(Self {
                rename: Some(rename.to_string()),
                module_name: module.to_string(),
            }),
            None => Ok(Self {
                rename: None,
                module_name: str.to_string(),
            }),
        }
    }
}

#[derive(Default)]
pub struct MethodAttr {
    pub module_name: String,
    pub type_name: String,
    pub rename: Option<String>,
}

impl syn::parse::Parse for MethodAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "missing \"module_name::type_name\" attribute",
            ));
        }

        let lit = input.parse::<syn::LitStr>()?;
        let str = lit.value();
        let parts: Vec<&str> = str.split("::").collect();

        match parts.as_slice() {
            // Case: "module::type"
            [module, type_name] => Ok(Self {
                module_name: module.to_string(),
                type_name: type_name.to_string(),
                rename: None,
            }),
            // Case: "module::type::rename"
            [module, type_name, rename] => Ok(Self {
                module_name: module.to_string(),
                type_name: type_name.to_string(),
                rename: Some(rename.to_string()),
            }),
            // Invalid cases
            _ => Err(syn::Error::new(
                lit.span(),
                "expected \"module::type\" or \"module::type::rename\"",
            )),
        }
    }
}

// pub fn parse_method_attr(attrs: &[syn::Attribute]) -> Result<Option<String>, TokenStream> {
//     for attr in attrs {
//         if attr.path().is_ident("rename") {
//             match attr.parse_args::<syn::LitStr>() {
//                 Ok(rename) => return Ok(Some(rename.value())),
//                 Err(err) => return Err(TokenStream::from(err.to_compile_error())),
//             }
//         }
//     }
//     Ok(None)
// }
