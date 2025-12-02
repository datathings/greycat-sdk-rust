use anyhow::Context;

use crate::Deserialize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AbiFunctions {
    functions: Box<[AbiFunction]>,
}

impl Deserialize for AbiFunctions {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let functions_size = de.read_u64().context("abi functions: size")?;
        if functions_size == 0 {
            return Ok(Self::default());
        }

        let nb_functions = de.read_u32().context("abi functions: count")?;
        if nb_functions == 0 {
            return Ok(Self::default());
        }

        let functions = {
            let mut functions = Vec::with_capacity(nb_functions as usize);
            for i in 0..nb_functions {
                let function =
                    AbiFunction::deserialize(de).with_context(|| format!("abi function {i}"))?;
                functions.push(function);
            }
            functions.into_boxed_slice()
        };

        Ok(Self { functions })
    }
}

impl std::ops::Index<u32> for AbiFunctions {
    type Output = AbiFunction;

    #[inline(always)]
    fn index(&self, index: u32) -> &Self::Output {
        &self.functions[index as usize]
    }
}

impl std::ops::Index<usize> for AbiFunctions {
    type Output = AbiFunction;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.functions[index]
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AbiFunction {
    pub module: u32,
    pub ty: u32,
    pub name: u32,
    pub lib: u32,
    pub arity: u32,
    pub return_type: u32,
    pub return_nullable: bool,
    pub params: Box<[AbiFnParam]>,
}

impl Deserialize for AbiFunction {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let module = de.read_vu32().context("abi fn: module")?;
        let ty = de.read_vu32().context("abi fn: type")?;
        let name = de.read_vu32().context("abi fn: name")?;
        let lib = de.read_vu32().context("abi fn: lib")?;
        let arity = de.read_vu32().context("abi fn: arity")?; // TODO: cannot be more than a u8 in the compiler, should be changed to u8
        let params = {
            let mut params = Vec::with_capacity(arity as usize);
            for i in 0..arity {
                let param = AbiFnParam::deserialize(de).with_context(|| {
                    if ty == 0 {
                        format!("abi fn {module}::{name}: param {i}")
                    } else {
                        format!("abi fn {module}::{ty}::{name}: param {i}")
                    }
                })?;
                params.push(param);
            }
            params.into_boxed_slice()
        };
        let return_type = de.read_vu32()?;
        let flags = de.read_u8()?;
        let return_nullable = (flags & 1) != 0;

        Ok(Self {
            module,
            ty,
            name,
            lib,
            arity,
            params,
            return_type,
            return_nullable,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AbiFnParam {
    pub name: u32,
    pub ty: u32,
    pub is_nullable: bool,
}

impl Deserialize for AbiFnParam {
    fn deserialize<D>(de: &mut D) -> crate::Result<Self>
    where
        D: crate::Deserializer,
    {
        let is_nullable = de.read_u8().context("abi fn param: nullable")? == 1;
        let ty = de.read_vu32().context("abi fn param: type")?;
        let name = de.read_vu32().context("abi fn param: name")?;

        Ok(Self {
            name,
            ty,
            is_nullable,
        })
    }
}
