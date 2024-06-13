use crate::{abi::AbiSymbol, value::Value};

#[derive(Debug)]
pub enum GcString<'abi> {
    Symbol(AbiSymbol<'abi>),
    String(String),
}

impl<'abi> From<GcString<'abi>> for Value<'abi> {
    #[inline]
    fn from(value: GcString<'abi>) -> Self {
        match value {
            GcString::Symbol(s) => Self::Symbol(s),
            GcString::String(s) => Self::String(s),
        }
    }
}
