use anyhow::Context;

use crate::{deserialize::Deserialize};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AbiHeaders {
    pub major: u16,
    pub magic: u16,
    pub version: u32,
    pub crc: u64,
}

impl Deserialize for AbiHeaders {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let major = de.read_u16().context("abi header: major")?;
        let magic = de.read_u16().context("abi header: magic")?;
        let version = de.read_u32().context("abi header: version")?;
        let crc = de.read_u64().context("abi header: crc")?;
        Ok(Self {
            major,
            magic,
            version,
            crc,
        })
    }
}
