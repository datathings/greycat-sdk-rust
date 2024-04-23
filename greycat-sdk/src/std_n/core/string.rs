use crate::abi::AbiSymbol;

pub enum GcString<'abi> {
    Symbol(AbiSymbol<'abi>),
    String(String),
}

