use std::{fs::File, io::BufReader, path::Path};

use anyhow::Context;

use crate::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Abi {
    pub headers: AbiHeaders,
    pub symbols: AbiSymbols,
    pub types: AbiTypes,
    pub functions: AbiFunctions,
}

impl Abi {
    /// Deserializes an `Abi` from the local `gcdata/abi` file if found
    pub fn from_local_file() -> Result<Self> {
        let filepath = Path::new("gcdata").join("abi");
        let file = File::open(filepath)?;
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }
}

impl Deserialize for Abi {
    fn deserialize<D>(de: &mut D) -> Result<Self>
    where
        D: Deserializer,
    {
        let headers = AbiHeaders::deserialize(de).context("abi headers")?;
        let symbols = AbiSymbols::deserialize(de).context("abi symbols")?;
        let types = AbiTypes::deserialize(de).context("abi types")?;
        let functions = AbiFunctions::deserialize(de).context("abi functions")?;
        Ok(Self {
            headers,
            symbols,
            types,
            functions,
        })
    }
}
