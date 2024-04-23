use std::rc::Rc;
use std::io::Write;
use byteorder::WriteBytesExt;
use anyhow::Result;

use crate::abi::{Abi, AbiType};
use crate::serialize::*;
use crate::primitive;
use crate::varint::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcEnum<'abi> {
    pub ty: Rc<AbiType>,
    pub offset: u32,
    pub key: &'abi str,
}

impl std::hash::Hash for GcEnum<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.mapped_abi_type_offset.hash(state);
        self.offset.hash(state);
    }
}

impl serde::Serialize for GcEnum<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut fqn = self.ty.fqn();
        fqn.push_str("::");
        fqn.push_str(self.key);

        serializer.serialize_str(&fqn)
    }
}

impl AbiSerialize for GcEnum<'_> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::ENUM)?;
        let mut n = writer.write_vu32(self.ty.mapped_abi_type_offset)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        let n = writer.write_vu32(self.offset)?;
        Ok(n)
    }
}

impl std::fmt::Debug for GcEnum<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.ty.fqn(), self.key)
    }
}
