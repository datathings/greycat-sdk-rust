use anyhow::Context as _;
use serde::Serialize;

use crate::{AbiSymbols, Deserialize};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AbiTypes {
    types: Box<[AbiType]>,
}

impl Deserialize for AbiTypes {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let types_size = de.read_u64().context("abi types: size")?;
        if types_size == 0 {
            return Ok(Self::default());
        }

        let nb_types = de.read_u32().context("abi types: count")?;
        if nb_types == 0 {
            return Ok(Self::default());
        }

        let _nb_attrs = de.read_u32().context("abi types: nb attributes")?;
        let types = {
            let mut types = Vec::with_capacity(nb_types as usize);
            for i in 0..nb_types {
                let ty = AbiType::deserialize(de)
                    .with_context(|| format!("unable to deserialize type #{i}"))?;
                types.push(ty);
            }
            types.into_boxed_slice()
        };

        Ok(Self { types })
    }
}

impl std::ops::Deref for AbiTypes {
    type Target = [AbiType];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.types
    }
}

impl std::ops::Index<u32> for AbiTypes {
    type Output = AbiType;

    #[inline(always)]
    fn index(&self, index: u32) -> &Self::Output {
        &self.types[index as usize]
    }
}

impl std::ops::Index<usize> for AbiTypes {
    type Output = AbiType;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.types[index]
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AbiTypeName<'a> {
    module: &'a str,
    ty: &'a str,
}

impl Serialize for AbiTypeName<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}::{}", self.module, self.ty.replace(",", ", ")))
    }
}

impl std::fmt::Debug for AbiTypeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.module, self.ty.replace(",", ", "))
    }
}

impl std::fmt::Display for AbiTypeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.module, self.ty.replace(",", ", "))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AbiType {
    pub module: u32,
    pub name: u32,
    pub lib_name: u32,
    pub generic_abi_type: u32,
    pub g1: u32,
    pub g2: u32,
    pub parent_type_id: u32,
    pub companion_type_id: u32,
    pub attributes_len: u32,
    pub attributes_offset: u32,
    pub mapped_prog_type_offset: u32,
    pub mapped_abi_type_offset: u32,
    pub masked_abi_type_offset: u32,
    pub nullable_nb_bytes: u32,
    pub flags: u8,
    pub attrs: Box<[AbiAttr]>,
}

impl AbiType {
    pub fn display_name<'a>(&self, symbols: &'a AbiSymbols) -> AbiTypeName<'a> {
        AbiTypeName {
            module: &symbols[self.module],
            ty: &symbols[self.name],
        }
    }

    #[inline(always)]
    pub fn is_native(&self) -> bool {
        (self.flags & 1) != 0
    }

    #[inline(always)]
    pub fn is_abstract(&self) -> bool {
        (self.flags & (1 << 1)) != 0
    }

    #[inline(always)]
    pub fn is_enum(&self) -> bool {
        (self.flags & (1 << 2)) != 0
    }

    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        (self.flags & (1 << 3)) != 0
    }

    #[inline(always)]
    pub fn is_ambiguous(&self) -> bool {
        (self.flags & (1 << 4)) != 0
    }

    #[inline(always)]
    pub fn is_volatile(&self) -> bool {
        (self.flags & (1 << 5)) != 0
    }
}

impl Deserialize for AbiType {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let module = de.read_vu32().context("abi type: module")?;
        let name = de.read_vu32().context("abi type: name")?;
        let lib_name = de.read_vu32().context("abi type: lib_name")?;
        let generic_abi_type = de.read_vu32().context("abi type: generic_abi_type")?;
        let g1 = de.read_vu32().context("abi type: g1")?;
        let g2 = de.read_vu32().context("abi type: g2")?;
        let parent_type_id = de.read_vu32().context("abi type: parent_type_id")?;
        let companion_type_id = de.read_vu32().context("abi type: companion_type_id")?;
        let attributes_len = de.read_vu32().context("abi type: attributes_len")?;
        let attributes_offset = de.read_vu32().context("abi type: attributes_offset")?;
        let mapped_prog_type_offset = de
            .read_vu32()
            .context("abi type: mapped_prog_type_offset")?;
        let mapped_abi_type_offset = de.read_vu32().context("abi type: mapped_abi_type_offset")?;
        let masked_abi_type_offset = de.read_vu32().context("abi type: masked_abi_type_offset")?;
        let nullable_nb_bytes = de.read_vu32().context("abi type: nullable_nb_bytes")?;
        let flags = de.read_u8().context("abi type: flags")?;
        let attrs = {
            let mut attrs = Vec::with_capacity(attributes_len as usize);
            for i in 0..attributes_len {
                let attr = AbiAttr::deserialize(de)
                    .with_context(|| format!("abi type {mapped_abi_type_offset}: attribute {i}"))?;
                attrs.push(attr);
            }
            attrs.into_boxed_slice()
        };

        Ok(Self {
            module,
            name,
            lib_name,
            generic_abi_type,
            g1,
            g2,
            parent_type_id,
            companion_type_id,
            attributes_len,
            attributes_offset,
            mapped_prog_type_offset,
            mapped_abi_type_offset,
            masked_abi_type_offset,
            nullable_nb_bytes,
            flags,
            attrs,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AbiAttr {
    pub name: u32,
    pub abi_type: u32,
    pub prog_type_offset: u32,
    pub mapped_any_offset: u32,
    pub mapped_att_offset: u32,
    pub sbi_type: u8,
    pub precision: u8,
    pub flags: u8,
}

impl AbiAttr {
    #[inline(always)]
    pub fn nullable(&self) -> bool {
        (self.flags & 1) != 0
    }

    #[inline(always)]
    pub fn mapped(&self) -> bool {
        (self.flags & (1 << 1)) != 0
    }
}

impl Deserialize for AbiAttr {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let name = de.read_vu32().context("name")?;
        let abi_type = de.read_vu32().context("abi_type")?;
        let prog_type_offset = de.read_vu32().context("prog_type_offset")?;
        let mapped_any_offset = de.read_vu32().context("mapped_any_offset")?;
        let mapped_att_offset = de.read_vu32().context("mapped_att_offset")?;
        let sbi_type = de.read_u8().context("sbi_type")?;
        let precision = de.read_u8().context("precision")?;
        let flags = de.read_u8().context("flags")?;

        Ok(Self {
            name,
            abi_type,
            prog_type_offset,
            mapped_any_offset,
            mapped_att_offset,
            sbi_type,
            precision,
            flags,
        })
    }
}
