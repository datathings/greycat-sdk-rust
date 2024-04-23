use byteorder::{WriteBytesExt, LE};
use ordered_float::OrderedFloat;
use serde::Serialize;

use crate::serialize::AbiSerialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
pub struct Float(pub OrderedFloat<f64>);

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self(OrderedFloat(value as f64))
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
}

impl std::ops::Deref for Float {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

impl AbiSerialize for Float {
    fn write_to<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        abi: &crate::prelude::Abi,
    ) -> anyhow::Result<usize> {
        writer.write_u8(crate::primitive::FLOAT)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        _abi: &crate::prelude::Abi,
    ) -> anyhow::Result<usize> {
        writer.write_f64::<LE>(self.0 .0)?;
        Ok(8)
    }
}
