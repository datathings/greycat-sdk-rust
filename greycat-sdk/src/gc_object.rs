use anyhow::Result;
use byteorder::WriteBytesExt;
use std::cell::{Ref, RefCell};
use std::io::Write;
use std::rc::Rc;

use crate::abi::{Abi, AbiType};
use crate::deserialize::AbiDeserialize;
use crate::prelude::TypeLoader;
use crate::primitive;
use crate::serialize::*;
use crate::value::Value;
use crate::varint::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcObject<'abi> {
    pub ty: Rc<AbiType>,
    pub values: Option<RefCell<Box<[Value<'abi>]>>>,
}

impl<'abi> GcObject<'abi> {
    pub fn new<T: Into<Box<[Value<'abi>]>>>(ty: Rc<AbiType>, values: Option<T>) -> Self {
        Self {
            ty,
            values: values.map(|values| RefCell::new(values.into())),
        }
    }

    /// Replaces a value in this instance.
    ///
    /// This method can panic if the given `index` is not in the bounds of the instance
    /// values.
    pub fn set_value(&self, index: usize, value: Value<'abi>) {
        if let Some(values) = &self.values {
            let mut values = values.borrow_mut();
            values[index] = value;
        }
    }

    pub fn get_value(&self, index: usize) -> Option<RefValue<'_, 'abi>> {
        if let Some(values) = &self.values {
            let values = values.borrow();
            let value = RefValue {
                inner: values,
                index,
            };
            Some(value)
        } else {
            None
        }
    }
}

impl<'abi, R> TypeLoader for R
where
    R: std::io::Read,
{
    fn load(&mut self, ty: Rc<AbiType>, abi: &Abi) -> Result<Value> {
        let value = AbiDeserialize::read_typed_object(self, ty, abi)?;
        Ok(Value::Obj(value))
    }
}

impl AbiSerialize for GcObject<'_> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(self.ty.mapped_abi_type_offset)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match (self.ty.attrs.as_ref(), self.values.as_ref()) {
            (None, None) => Ok(0),
            (None, Some(values)) => anyhow::bail!(
                "object '{}' has 0 attribute defined but {} values",
                self.ty.name,
                values.borrow().len()
            ),
            (Some(attrs), None) => anyhow::bail!(
                "object '{}' has {} attributes defined but 0 value",
                self.ty.name,
                attrs.len()
            ),
            (Some(attrs), Some(values)) => {
                let mut n = 0;

                if self.ty.nullable_nb_bytes > 0 {
                    let mut nullable_bitset = vec![0u8; self.ty.nullable_nb_bytes as usize];
                    let mut nullable_offset = 0;
                    for (att, value) in attrs.iter().zip(values.borrow().iter()) {
                        if att.nullable {
                            match value {
                                Value::Null => {
                                    gc_object_set_null(&mut nullable_bitset, nullable_offset)
                                }
                                _ => gc_object_set_not_null(&mut nullable_bitset, nullable_offset),
                            }
                            nullable_offset += 1;
                        }
                    }
                    writer.write_all(&nullable_bitset)?;
                    n += self.ty.nullable_nb_bytes as usize;
                }

                for (attr, value) in attrs.iter().zip(values.borrow().iter()) {
                    if attr.nullable && matches!(value, Value::Null) {
                        // skip nullable attr that is actually 'null'
                        continue;
                    }

                    match attr.sbi_type {
                        primitive::BOOL => match value {
                            Value::Bool(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            v => {
                                anyhow::bail!(
                                    "expected attribute '{}' in '{}' to be a bool, got {v}",
                                    attr.name,
                                    self.ty.name,
                                )
                            }
                        },
                        primitive::CHAR => match value {
                            Value::Char(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            v => {
                                anyhow::bail!(
                                    "expected attribute '{}' in '{}' to be a char, got {v}",
                                    attr.name,
                                    self.ty.name,
                                )
                            }
                        },
                        primitive::INT => match value {
                            Value::Int(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            v => {
                                anyhow::bail!(
                                    "expected attribute '{}' in '{}' to be an int, got {v}",
                                    attr.name,
                                    self.ty.name,
                                )
                            }
                        },
                        primitive::FLOAT => match value {
                            Value::Float(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            v => {
                                anyhow::bail!(
                                    "expected attribute '{}' in '{}' to be a float, got {v}",
                                    attr.name,
                                    self.ty.name,
                                )
                            }
                        },
                        primitive::OBJECT => match value {
                            Value::Obj(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            Value::Array(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            Value::Map(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            Value::String(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            Value::Symbol(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            Value::Enum(v) => {
                                n += v.write_raw_to(writer, abi)?;
                            }
                            v => {
                                anyhow::bail!(
                                    "expected attribute '{}' in '{}' to be an object, got {v}",
                                    attr.name,
                                    self.ty.name,
                                )
                            }
                        },
                        primitive::UNDEFINED => {
                            n += value.write_to(writer, abi)?;
                        }
                        primitive::NULL => (),
                        _ => {
                            n += value.write_raw_to(writer, abi)?;
                        }
                    }
                }

                Ok(n)
            }
        }
    }
}

impl std::fmt::Debug for GcObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct(&self.ty.fqn());
        if let (Some(attrs), Some(values)) = (self.ty.attrs.as_ref(), self.values.as_ref()) {
            for (attr, value) in attrs.iter().zip(values.borrow().iter()) {
                // TODO do better for GcObject debug
                s.field(&format!("#{}", attr.name), value);
            }
        }
        s.finish()
    }
}

impl std::hash::Hash for GcObject<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.mapped_abi_type_offset.hash(state);
        if let Some(values) = self.values.as_ref() {
            for v in values.borrow().as_ref() {
                v.hash(state);
            }
        }
    }
}

impl serde::Serialize for GcObject<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        match self.ty.attrs.as_ref() {
            Some(attrs) => {
                let mut s = serializer.serialize_map(Some(attrs.len()))?;
                // s.serialize_entry("_type", &self.ty.fqn())?;
                for (attr, value) in attrs.iter().zip(self.values.iter()) {
                    // TODO do better for GcObject debug
                    s.serialize_entry(&format!("#{}", attr.name), value)?;
                }
                s.end()
            }
            None => serializer.serialize_map(None)?.end(),
        }
    }
}

pub struct RefValue<'r, 'abi> {
    inner: Ref<'r, Box<[Value<'abi>]>>,
    index: usize,
}

impl<'r, 'abi> RefValue<'r, 'abi> {
    pub fn get(&self) -> &Value<'abi> {
        &self.inner[self.index]
    }
}

impl<'r, 'abi> Eq for RefValue<'r, 'abi> {}

impl<'r, 'abi> PartialEq for RefValue<'r, 'abi> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<'r, 'abi> std::fmt::Debug for RefValue<'r, 'abi> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<'r, 'abi> std::ops::Deref for RefValue<'r, 'abi> {
    type Target = Value<'abi>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

const GC_OBJECT_BITSET_BLOCK_SIZE: usize = 8;

#[inline]
pub(crate) fn attr_is_null(bitset: &[u8], offset: usize) -> bool {
    ((bitset[offset >> 3] >> (offset & (GC_OBJECT_BITSET_BLOCK_SIZE - 1))) & 1) == 0
}

pub(crate) fn gc_object_set_null(bitset: &mut [u8], offset: usize) {
    // Find the index of the slice element containing the bit we want to set to 0
    let bitset_index: usize = offset >> 3; // Equivalent to integer division by 8

    // Find the position of the bit within the slice element
    let bit_position: usize = offset & (GC_OBJECT_BITSET_BLOCK_SIZE - 1); // Equivalent to offset % 8

    // Clear the bit at the specified position to 0 using bitwise AND with the complement of 1 at that position
    bitset[bitset_index] &= !(1 << bit_position);
}

pub(crate) fn gc_object_set_not_null(bitset: &mut [u8], offset: usize) {
    // Find the index of the slice element containing the bit we want to set to 1
    let bitset_index: usize = offset >> 3; // Equivalent to integer division by 8

    // Find the position of the bit within the slice element
    let bit_position: usize = offset & (GC_OBJECT_BITSET_BLOCK_SIZE - 1); // Equivalent to offset % 8

    // Set the bit at the specified position to 1 using bitwise OR with 1 at that position
    bitset[bitset_index] |= 1 << bit_position;
}
