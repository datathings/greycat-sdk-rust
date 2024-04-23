use std::io::Write;

use anyhow::Result;

use crate::prelude::Abi;

pub trait AbiSerialize {
    /// Serializes the value with its headers to the given writer
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize>;

    /// Serializes the value without its headers to the given writer
    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize>;
}
