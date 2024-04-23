use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{WriteBytesExt, LE};
use serde::Serialize;

use crate::abi::{Abi, RequestHeaders};
use crate::deserialize::AbiDeserialize;
use crate::gc_enum::GcEnum;
use crate::gc_object::GcObject;
use crate::prelude::AbiSymbol;
use crate::serialize::AbiSerialize;
use crate::std_n::core::Float;
use crate::varint::{VarintRead, VarintWrite};
use crate::{primitive, std_n};

#[derive(Clone, Serialize)]
pub struct HeaderValue<'abi> {
    pub headers: RequestHeaders,
    pub value: Value<'abi>,
}

impl<'abi> HeaderValue<'abi> {
    pub fn from_reader<T>(mut reader: T, abi: &'abi Abi) -> Result<Self>
    where
        T: Read,
    {
        use crate::abi::RequestHeadersRead;

        let headers = reader.read_request_headers()?;
        let value = reader.read_value(abi)?;

        Ok(Self { headers, value })
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[serde(untagged)]
pub enum Value<'abi> {
    Null,
    Int(i64),
    Float(std_n::core::Float),
    Char(char),
    Bool(bool),
    Array(Vec<Value<'abi>>),
    Map(BTreeMap<Value<'abi>, Value<'abi>>),
    Symbol(AbiSymbol<'abi>),
    Node(std_n::core::Node),
    NodeTime(std_n::core::NodeTime),
    NodeIndex(std_n::core::NodeIndex),
    NodeList(std_n::core::NodeList),
    NodeGeo(std_n::core::NodeGeo),
    Geo(std_n::core::Geo),
    Time(std_n::core::Time),
    Duration(std_n::core::Duration),
    String(String),
    Enum(GcEnum<'abi>),
    Obj(GcObject<'abi>),
}

impl std::default::Default for Value<'_> {
    fn default() -> Self {
        Self::Null
    }
}

impl<'abi> From<&serde_json::Value> for Value<'abi> {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(v) => Value::Bool(*v),
            serde_json::Value::Number(v) if v.is_f64() => {
                Value::Float(Float::from(v.as_f64().unwrap()))
            }
            serde_json::Value::Number(v) => Value::Int(v.as_i64().unwrap()),
            serde_json::Value::String(v) => Value::String(v.clone()),
            serde_json::Value::Array(v) => Value::Array(v.iter().map(Value::from).collect()),
            serde_json::Value::Object(v) => {
                Value::Map(BTreeMap::from_iter(v.iter().map(|(key, value)| {
                    (Value::String(key.clone()), Value::from(value))
                })))
            }
        }
    }
}

impl<'abi> AbiSerialize for Value<'abi> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match self {
            Value::Null => {
                writer.write_u8(primitive::NULL)?;
                Ok(1)
            }
            Value::Int(v) => v.write_to(writer, abi),
            Value::Float(v) => v.write_to(writer, abi),
            Value::Bool(v) => v.write_to(writer, abi),
            Value::Char(v) => v.write_to(writer, abi),
            Value::Array(v) => v.write_to(writer, abi),
            Value::Map(v) => v.write_to(writer, abi),
            Value::Symbol(v) => v.0.write_to(writer, abi),
            Value::String(v) => v.write_to(writer, abi),
            Value::Node(v) => v.write_to(writer, abi),
            Value::NodeTime(v) => v.write_to(writer, abi),
            Value::NodeIndex(v) => v.write_to(writer, abi),
            Value::NodeList(v) => v.write_to(writer, abi),
            Value::NodeGeo(v) => v.write_to(writer, abi),
            Value::Geo(v) => v.write_to(writer, abi),
            Value::Time(v) => v.write_to(writer, abi),
            Value::Duration(v) => v.write_to(writer, abi),
            Value::Enum(v) => v.write_to(writer, abi),
            Value::Obj(v) => v.write_to(writer, abi),
        }
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match self {
            Value::Null => Ok(0),
            Value::Int(v) => v.write_raw_to(writer, abi),
            Value::Float(v) => v.write_raw_to(writer, abi),
            Value::Bool(v) => v.write_raw_to(writer, abi),
            Value::Char(v) => v.write_raw_to(writer, abi),
            Value::Array(v) => v.write_raw_to(writer, abi),
            Value::Map(v) => v.write_raw_to(writer, abi),
            Value::Symbol(v) => v.0.write_raw_to(writer, abi),
            Value::String(v) => v.write_raw_to(writer, abi),
            Value::Node(v) => v.write_raw_to(writer, abi),
            Value::NodeTime(v) => v.write_raw_to(writer, abi),
            Value::NodeIndex(v) => v.write_raw_to(writer, abi),
            Value::NodeList(v) => v.write_raw_to(writer, abi),
            Value::NodeGeo(v) => v.write_raw_to(writer, abi),
            Value::Geo(v) => v.write_raw_to(writer, abi),
            Value::Time(v) => v.write_raw_to(writer, abi),
            Value::Duration(v) => v.write_raw_to(writer, abi),
            Value::Enum(v) => v.write_raw_to(writer, abi),
            Value::Obj(v) => v.write_raw_to(writer, abi),
        }
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::Int(_) => f.write_str("int"),
            Value::Float(_) => f.write_str("float"),
            Value::Char(_) => f.write_str("char"),
            Value::Bool(_) => f.write_str("bool"),
            Value::Array(_) => f.write_str("Array"),
            Value::Map(_) => f.write_str("Map"),
            Value::Symbol(_) => f.write_str("symbol"),
            Value::Node(_) => f.write_str("node"),
            Value::NodeTime(_) => f.write_str("nodeTime"),
            Value::NodeIndex(_) => f.write_str("nodeIndex"),
            Value::NodeList(_) => f.write_str("nodeList"),
            Value::NodeGeo(_) => f.write_str("nodeGeo"),
            Value::Geo(_) => f.write_str("geo"),
            Value::Time(_) => f.write_str("time"),
            Value::Duration(_) => f.write_str("duration"),
            Value::String(_) => f.write_str("String"),
            Value::Enum(v) => write!(f, "Enum#{}", v.ty.mapped_abi_type_offset), // TODO find better?
            Value::Obj(v) => write!(f, "Object#{}", v.ty.mapped_abi_type_offset), // TODO find better?
        }
    }
}

impl<'a> std::fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::Int(v) => v.fmt(f),
            Value::Float(v) => v.fmt(f),
            Value::Char(v) => v.fmt(f),
            Value::Bool(v) => v.fmt(f),
            Value::Array(v) => v.fmt(f),
            Value::Map(v) => v.fmt(f),
            Value::Symbol(v) => v.fmt(f),
            Value::Node(v) => v.fmt(f),
            Value::NodeTime(v) => v.fmt(f),
            Value::NodeIndex(v) => v.fmt(f),
            Value::NodeList(v) => v.fmt(f),
            Value::NodeGeo(v) => v.fmt(f),
            Value::Geo(v) => v.fmt(f),
            Value::Time(v) => v.fmt(f),
            Value::Duration(v) => v.fmt(f),
            Value::String(v) => v.fmt(f),
            Value::Enum(v) => v.fmt(f),
            Value::Obj(v) => v.fmt(f),
        }
    }
}

impl<T: AbiSerialize, const N: usize> AbiSerialize for [T; N] {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.array)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        let mut n = writer.write_vu32(self.len() as u32)?;
        for elem in self {
            n += elem.write_to(writer, abi)?;
        }
        Ok(n)
    }
}

impl<'a, T: AbiSerialize> AbiSerialize for &'a [T] {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.array)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        let mut n = writer.write_vu32(self.len() as u32)?;
        for elem in *self {
            n += elem.write_to(writer, abi)?;
        }
        Ok(n)
    }
}

impl<'a, T: AbiSerialize> AbiSerialize for &'a Vec<T> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.array)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        let mut n = writer.write_vu32(self.len() as u32)?;
        for elem in *self {
            n += elem.write_to(writer, abi)?;
        }
        Ok(n)
    }
}

impl<'a, K: AbiSerialize, V: AbiSerialize> AbiSerialize for &'a HashMap<K, V> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.map)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        let mut n = writer.write_vu32(self.len() as u32)?;
        for (key, value) in *self {
            n += key.write_to(writer, abi)?;
            n += value.write_to(writer, abi)?;
        }
        Ok(n)
    }
}

impl<'a, K: AbiSerialize, V: AbiSerialize> AbiSerialize for &'a BTreeMap<K, V> {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.map)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        let mut n = writer.write_vu32(self.len() as u32)?;
        for (key, value) in *self {
            n += key.write_to(writer, abi)?;
            n += value.write_to(writer, abi)?;
        }
        Ok(n)
    }
}

impl AbiSerialize for i64 {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::INT)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        let n = writer.write_vi64(*self)?;
        Ok(n)
    }
}

impl AbiSerialize for bool {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::BOOL)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        writer.write_u8(if *self { 1 } else { 0 })?;
        Ok(1)
    }
}

impl AbiSerialize for char {
    /// Serializes a `char` with its header.
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::CHAR)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        if self.is_ascii() {
            let mut buf = [0u8; 1];
            self.encode_utf8(&mut buf);
            writer.write_all(&buf)?;
            return Ok(1);
        }
        anyhow::bail!("'{self}' is not an ASCII char")
    }
}

impl AbiSerialize for f64 {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::FLOAT)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        writer.write_f64::<LE>(*self)?;
        Ok(8)
    }
}

struct Symbol(u32);

impl AbiSerialize for Symbol {
    #[inline(always)]
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::STR_LIT)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    #[inline(always)]
    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        let n = writer.write_vu32((self.0 << 1) | 1)?;
        Ok(n)
    }
}

struct AnyString<'a>(&'a str);

impl AbiSerialize for AnyString<'_> {
    #[inline(always)]
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(primitive::OBJECT)?;
        let mut n = writer.write_vu32(abi.types.core.string)?;
        n += self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    #[inline(always)]
    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        let str_bytes = self.0.as_bytes();
        let str_len = str_bytes.len();
        let n = writer.write_vu32((str_len as u32) << 1)?;
        writer.write_all(str_bytes)?;
        Ok(n + str_bytes.len())
    }
}

impl<'a> AbiSerialize for &'a str {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match abi.symbols.get(self) {
            Some(off) => Symbol(off).write_to(writer, abi),
            None => AnyString(self).write_to(writer, abi),
        }
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match abi.symbols.get(self) {
            Some(off) => Symbol(off).write_raw_to(writer, abi),
            None => AnyString(self).write_raw_to(writer, abi),
        }
    }
}

impl AbiSerialize for String {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match abi.symbols.get(self) {
            Some(off) => Symbol(off).write_to(writer, abi),
            None => AnyString(self).write_to(writer, abi),
        }
    }

    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        match abi.symbols.get(self) {
            Some(off) => Symbol(off).write_raw_to(writer, abi),
            None => AnyString(self).write_raw_to(writer, abi),
        }
    }
}

pub trait GcEnumRead {
    fn read_enum<'abi>(&mut self, abi: &'abi Abi) -> Result<GcEnum<'abi>>;
}

impl<T: Read> GcEnumRead for T {
    fn read_enum<'abi>(&mut self, abi: &'abi Abi) -> Result<GcEnum<'abi>> {
        let enum_id = self.read_vu32()?;
        let variant = self.read_vu32()?;
        if let Some(ty) = abi.types.get(enum_id) {
            if let Some(attrs) = ty.attrs.as_ref() {
                let key = attrs[variant as usize].name;
                let offset = attrs[variant as usize].mapped_att_offset;
                return Ok(GcEnum {
                    ty,
                    key: &abi.symbols[key],
                    offset,
                });
            } else {
                anyhow::bail!("enum '{}' has no field at offset {}", ty.name, variant);
            }
        }
        anyhow::bail!("unknown enum id {enum_id}")
    }
}

// pub trait GcObjectRead {
//     /// Reads the object type first, then the object
//     fn read_object<'abi>(&mut self, abi: &'abi Abi) -> Result<GcObject<'abi>>;

//     /// Reads the object using the given `type_id` for deserialisation
//     fn read_typed_object<'abi>(&mut self, type_id: u32, abi: &'abi Abi) -> Result<GcObject<'abi>>;
// }

// impl<T: Read> GcObjectRead for T {
//     fn read_object<'abi>(&mut self, abi: &'abi Abi) -> Result<GcObject<'abi>> {
//         let type_id = self.read_vu32()?;
//         self.read_typed_object(type_id, abi)
//     }

//     fn read_typed_object<'abi>(&mut self, type_id: u32, abi: &'abi Abi) -> Result<GcObject<'abi>> {
//         let value = match type_id {
//             id if id == abi.types.core.string => {
//                 let len = self.read_vu32()?;
//                 if len & 1 == 1 {
//                     Value::Str(&abi.symbols[len >> 1])
//                 } else {
//                     let mut buf = vec![0u8; (len >> 1) as usize];
//                     self.read_exact(&mut buf[..])?;
//                     Value::String(String::from_utf8(buf)?)
//                 }
//             }
//             id if id == abi.types.core.array => {
//                 let len: u32 = self.read_vu32()?;
//                 let mut values = Vec::with_capacity(len as usize);
//                 for _ in 0..len {
//                     let value = self.read_value(abi)?;
//                     values.push(value);
//                 }
//                 Value::Array(values)
//             }
//             id if id == abi.types.core.map => {
//                 let len: u32 = self.read_vu32()?;
//                 // XXX this is a lie, if a Value::Object(GcObject) change its inner values
//                 // the hash won't be updated and therefore you won't be able to access that value
//                 // ever, which is an issue. I need to think about it. GreyCat does not allow non-primitives
//                 // as key in maps, I should do the same.
//                 #[allow(clippy::mutable_key_type)]
//                 let mut map = BTreeMap::new();
//                 for _ in 0..len {
//                     let key = self.read_value(abi)?;
//                     let value = self.read_value(abi)?;
//                     map.insert(key, value);
//                 }
//                 Value::Map(map)
//             }
//             n => match abi.types.get(n) {
//                 Some(ty) if ty.is_native => {
//                     // TODO use the registered factory for native types
//                     anyhow::bail!("native type {} has no deserializer implemented", ty.fqn())
//                 }
//                 Some(ty) => {
//                     let obj = self.read_object(abi)?;
//                     Value::Obj(obj)
//                 }
//                 None => anyhow::bail!("unhandled object type id {n}"),
//             },
//         };
//     }
// }
