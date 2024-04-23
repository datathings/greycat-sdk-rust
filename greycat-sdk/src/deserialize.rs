use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use byteorder::LE;

use crate::abi::{Abi, AbiSymbol, AbiType};
use crate::gc_enum::GcEnum;
use crate::gc_object::{attr_is_null, GcObject};
use crate::primitive;
use crate::std_n::core::{self, GcString};
use crate::value::Value;
use crate::varint::VarintRead;

pub trait AbiDeserialize<'abi> {
    /// Reads a varint 64bits to yield a `i64`
    fn read_int(&mut self) -> Result<i64>;
    /// Reads a little-endian 64bits float to yield an `f64`
    fn read_float(&mut self) -> Result<f64>;
    /// Reads a `u8` to yield a `bool` (0:false, !0:true)
    fn read_bool(&mut self) -> Result<bool>;
    /// Reads a `u32` to yield a `char`
    fn read_char(&mut self) -> Result<char>;
    /// Reads a varint 64bits to yield a `std_n::core::Time`
    fn read_time(&mut self) -> Result<core::Time>;
    /// Reads a varint 64bits to yield a `std_n::core::Duration`
    fn read_duration(&mut self) -> Result<core::Duration>;
    /// Reads a varint 64bits to yield a `std_n::core::Geo`
    fn read_geo(&mut self) -> Result<core::Geo>;
    /// Reads a varint 64bits to yield a `std_n::core::Node`
    fn read_node(&mut self) -> Result<core::Node>;
    /// Reads a varint 64bits to yield a `std_n::core::NodeGeo`
    fn read_nodegeo(&mut self) -> Result<core::NodeGeo>;
    /// Reads a varint 64bits to yield a `std_n::core::NodeList`
    fn read_nodelist(&mut self) -> Result<core::NodeList>;
    /// Reads a varint 64bits to yield a `std_n::core::NodeIndex`
    fn read_nodeindex(&mut self) -> Result<core::NodeIndex>;
    /// Reads a varint 64bits to yield a `std_n::core::NodeTime`
    fn read_nodetime(&mut self) -> Result<core::NodeTime>;
    /// Reads an unsigned varint 32bits to yield an `AbiSymbol`
    fn read_symbol(&mut self, abi: &'abi Abi) -> Result<AbiSymbol<'abi>>;
    /// Reads an unsigned varint 32bits. Based on the value it will either
    /// yield an `AbiSymbol` or a `String`
    fn read_string(&mut self, abi: &'abi Abi) -> Result<GcString<'abi>>;
    /// Reads a GreyCat string
    ///  - reads a `vu32` as length
    ///  - reads `length` bytes (which is the string's data)
    fn read_object_string(&mut self) -> Result<String>;
    /// Reads a GreyCat object
    ///  - reads a `vu32` as type id
    ///  - use the type id loader to read the object value
    fn read_object(&mut self, abi: &'abi Abi) -> Result<GcObject<'abi>>;
    /// Reads an object value using the given `ty` loader
    fn read_typed_object(&mut self, ty: Rc<AbiType>, abi: &'abi Abi) -> Result<GcObject<'abi>>;
    /// Reads a GreyCat enum
    ///  - reads a `vu32` as type id
    ///  - reads a `vu32` as enum field offset
    fn read_enum(&mut self, abi: &'abi Abi) -> Result<GcEnum<'abi>>;
    /// Reads a `vu32` as enum field offset and uses the given `en` id for the enum id
    fn read_typed_enum(&mut self, en: Rc<AbiType>, abi: &'abi Abi) -> Result<GcEnum<'abi>>;
    /// Reads a value by first reading a `u8` to get the value header type, then calls `read_value_header()` with it
    fn read_value(&mut self, abi: &'abi Abi) -> Result<Value<'abi>>;
    /// Reads a value using the given `header` byte to choose the right type loader
    fn read_value_header(&mut self, header: u8, abi: &'abi Abi) -> Result<Value<'abi>>;
}

impl<'abi, T> AbiDeserialize<'abi> for T
where
    T: std::io::Read,
{
    fn read_int(&mut self) -> Result<i64> {
        let value = self.read_vi64()?;
        Ok(value)
    }

    fn read_float(&mut self) -> Result<f64> {
        let value = byteorder::ReadBytesExt::read_f64::<LE>(self)?;
        Ok(value)
    }

    fn read_bool(&mut self) -> Result<bool> {
        let value = byteorder::ReadBytesExt::read_u8(self)?;
        Ok(value != 0)
    }

    fn read_char(&mut self) -> Result<char> {
        let charcode = byteorder::ReadBytesExt::read_u32::<LE>(self)?;
        let value = char::from_u32(charcode)
            .ok_or_else(|| anyhow!("invalid value {charcode} for a char"))?;
        Ok(value)
    }

    fn read_time(&mut self) -> Result<core::Time> {
        let value = self.read_vi64()?;
        Ok(core::Time(value))
    }

    fn read_duration(&mut self) -> Result<core::Duration> {
        let value = self.read_vi64()?;
        Ok(core::Duration(value))
    }

    fn read_geo(&mut self) -> Result<core::Geo> {
        let value = self.read_vu64()?;
        Ok(core::Geo(value))
    }

    fn read_node(&mut self) -> Result<core::Node> {
        let value = self.read_vu64()?;
        Ok(core::Node(value))
    }

    fn read_nodegeo(&mut self) -> Result<core::NodeGeo> {
        let value = self.read_vu64()?;
        Ok(core::NodeGeo(value))
    }

    fn read_nodeindex(&mut self) -> Result<core::NodeIndex> {
        let value = self.read_vu64()?;
        Ok(core::NodeIndex(value))
    }

    fn read_nodelist(&mut self) -> Result<core::NodeList> {
        let value = self.read_vu64()?;
        Ok(core::NodeList(value))
    }

    fn read_nodetime(&mut self) -> Result<core::NodeTime> {
        let value = self.read_vu64()?;
        Ok(core::NodeTime(value))
    }

    fn read_symbol(&mut self, abi: &'abi Abi) -> Result<AbiSymbol<'abi>> {
        let mut symb_id = self.read_vu32()?;
        symb_id >>= 1;
        Ok(AbiSymbol(&abi.symbols[symb_id]))
    }

    fn read_string(&mut self, abi: &'abi Abi) -> Result<GcString<'abi>> {
        let len = self.read_vu32()?;
        let value = if len & 1 == 1 {
            GcString::Symbol(AbiSymbol(&abi.symbols[len >> 1]))
        } else {
            let len = len >> 1;
            let mut buf = self.take(len as u64);
            let mut value = String::with_capacity(len as usize);
            buf.read_to_string(&mut value)?;
            GcString::String(value)
        };
        Ok(value)
    }

    fn read_object_string(&mut self) -> Result<String> {
        let len = self.read_vu32()?;
        let mut buf = self.take(len as u64);
        let mut value = String::with_capacity(len as usize);
        buf.read_to_string(&mut value)?;
        Ok(value)
    }

    fn read_object(&mut self, abi: &'abi Abi) -> Result<GcObject<'abi>> {
        let type_id = self.read_vu32()?;
        let ty = abi
            .types
            .get(type_id)
            .ok_or_else(|| anyhow!("unknown type with id '{type_id}'"))?;
        self.read_typed_object(ty, abi)
    }

    fn read_typed_object(&mut self, ty: Rc<AbiType>, abi: &'abi Abi) -> Result<GcObject<'abi>> {
        if ty.is_native {
            todo!()
        }

        // TODO maybe add a anyhow Context or a better error handling here, even though, this should be
        // safe to unwrap. Do more evol tests.
        let prog_type = abi.types.get(ty.mapped_abi_type_offset).unwrap();
        if let Some(attrs) = ty.attrs.as_ref() {
            let target_attrs_len = prog_type
                .attrs
                .as_ref()
                .map(|attrs| attrs.len())
                .unwrap_or(0);
            let mut values = vec![Value::default(); target_attrs_len];
            let mut nullable_bitset = vec![0u8; ty.nullable_nb_bytes as usize];
            self.read_exact(&mut nullable_bitset[..])?;
            let mut nullable_attr_offset = 0;

            for attr in attrs.iter() {
                if attr.nullable {
                    if attr_is_null(&nullable_bitset, nullable_attr_offset) {
                        nullable_attr_offset += 1;
                        continue;
                    }
                    nullable_attr_offset += 1;
                }
                let mut load_type = attr.sbi_type;
                if load_type == primitive::UNDEFINED {
                    load_type = byteorder::ReadBytesExt::read_u8(self)?;
                }
                let value = match load_type {
                    primitive::ENUM if attr.sbi_type == primitive::UNDEFINED => {
                        Value::Enum(self.read_enum(abi)?)
                    }
                    primitive::ENUM => {
                        let ty = &abi.types[attr.abi_type];
                        let prog_ty = &abi.types[ty.mapped_abi_type_offset];
                        let offset = self.read_vu32()?;
                        match ty.attrs.as_ref() {
                            Some(attrs) => {
                                let attr = &attrs[offset as usize];
                                let en = GcEnum {
                                    ty: Rc::clone(prog_ty),
                                    key: &abi.symbols[attr.name],
                                    offset: attr.mapped_att_offset,
                                };
                                Value::Enum(en)
                            }
                            None => bail!("trying to read enum field on a type with no attrs"),
                        }
                    }
                    primitive::OBJECT if attr.sbi_type == primitive::UNDEFINED => {
                        self.read_value(abi)?
                    }
                    primitive::OBJECT => {
                        let mut attr_obj_ty = &abi.types[attr.abi_type];
                        if attr_obj_ty.is_abstract {
                            // if the attr type is abstract, we need to determine the concrete type
                            let attr_type_id = self.read_vu32()?;
                            attr_obj_ty = &abi.types[attr_type_id];
                        }
                        Value::Obj(self.read_typed_object(Rc::clone(attr_obj_ty), abi)?)
                    }
                    n => self.read_value_header(n, abi)?,
                };
                if attr.mapped {
                    values[attr.mapped_att_offset as usize] = value;
                }
            }

            return Ok(GcObject {
                ty: prog_type,
                values: Some(RefCell::new(values.into_boxed_slice())),
            });
        }

        Ok(GcObject {
            ty: prog_type,
            values: None,
        })
    }

    fn read_enum(&mut self, abi: &'abi Abi) -> Result<GcEnum<'abi>> {
        let enum_id = self.read_vu32()?;
        let en = abi
            .types
            .get(enum_id)
            .ok_or_else(|| anyhow!("unknown enum id {enum_id}"))?;
        self.read_typed_enum(en, abi)
    }

    fn read_typed_enum(&mut self, en: Rc<AbiType>, abi: &'abi Abi) -> Result<GcEnum<'abi>> {
        let offset = self.read_vu32()?;
        let attrs = en
            .attrs
            .as_ref()
            .ok_or_else(|| anyhow!("enum '{}' has no attributes", &abi.symbols[en.name]))?;
        let key = attrs[offset as usize].name;
        let offset = attrs[offset as usize].mapped_att_offset;
        Ok(GcEnum {
            ty: en,
            key: &abi.symbols[key],
            offset,
        })
    }

    fn read_value(&mut self, abi: &'abi Abi) -> Result<Value<'abi>> {
        let header = byteorder::ReadBytesExt::read_u8(self)?;
        self.read_value_header(header, abi)
    }

    fn read_value_header(&mut self, header: u8, abi: &'abi Abi) -> Result<Value<'abi>> {
        let value = match header {
            primitive::INT => Value::Int(self.read_int()?),
            primitive::FLOAT => Value::Float(self.read_float()?.into()),
            primitive::BOOL => Value::Bool(self.read_bool()?),
            primitive::CHAR => Value::Char(self.read_char()?),
            primitive::NODE => Value::Node(self.read_node()?),
            primitive::NODE_TIME => Value::NodeTime(self.read_nodetime()?),
            primitive::NODE_INDEX => Value::NodeIndex(self.read_nodeindex()?),
            primitive::NODE_LIST => Value::NodeList(self.read_nodelist()?),
            primitive::NODE_GEO => Value::NodeGeo(self.read_nodegeo()?),
            primitive::GEO => Value::Geo(self.read_geo()?),
            primitive::TIME => Value::Time(self.read_time()?),
            primitive::DURATION => Value::Duration(self.read_duration()?),
            primitive::FN => todo!("fn ptr are not implemented yet"),
            primitive::STR_LIT => Value::Symbol(self.read_symbol(abi)?),
            primitive::ENUM => Value::Enum(self.read_enum(abi)?),
            primitive::OBJECT => Value::Obj(self.read_object(abi)?),
            n => anyhow::bail!("unknown primitive type {n}"),
        };

        Ok(value)
    }
}

// #[test]
// fn test() {
//     use crate::serialize::AbiSerialize;

//     let abi = Abi::new(&*std::fs::read("../gcdata/store/abi").unwrap(), None).unwrap();

//     let mut buf = vec![];
//     // buf.write_vu32(5).unwrap();
//     // buf.write_all(b"hello").unwrap();
//     "hello".write_to(&mut buf, &abi).unwrap();

//     let mut bytes = &*buf;

//     let value = bytes.read_value(&abi).unwrap();
//     assert_eq!(value, Value::Symbol(AbiSymbol("hello")));
// }
