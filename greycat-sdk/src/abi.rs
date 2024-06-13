use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Write;
use std::rc::Rc;
use std::{collections::HashMap, io::Read};

use anyhow::{anyhow, Result};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::ser::SerializeSeq;
use serde::Serialize;

use crate::library::Library;
use crate::prelude::{TypeFactory, TypeLoader};
use crate::serialize::AbiSerialize;
use crate::std::StdLibrary;
use crate::varint::VarintRead;

#[derive(Default)]
pub struct AbiBuilder {
    libraries: Option<Vec<Box<dyn Library>>>,
}

impl AbiBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_library<T>(mut self, library: T) -> Self
    where
        T: Library + 'static,
    {
        match self.libraries.as_mut() {
            Some(libraries) => libraries.push(Box::new(library)),
            None => self.libraries = Some(vec![Box::new(library)]),
        }
        self
    }

    pub fn with_libraries(mut self, libraries: Vec<Box<dyn Library>>) -> Self {
        match self.libraries.as_mut() {
            Some(libs) => libs.extend(libraries),
            None => self.libraries = Some(libraries),
        }
        self
    }

    pub fn build<T>(self, bytes: T) -> Result<Abi>
    where
        T: Read,
    {
        Abi::new(bytes, self.libraries)
    }
}

#[derive(Serialize)]
pub struct Abi {
    #[serde(flatten)]
    pub headers: AbiHeaders,
    #[serde(flatten)]
    pub symbols: AbiSymbols,
    #[serde(flatten)]
    pub types: AbiTypes,
    #[serde(flatten)]
    pub functions: AbiFunctions,
    #[serde(skip)]
    pub libraries: Vec<Box<dyn Library>>,
    #[serde(skip)]
    pub loaders: BTreeMap<&'static str, Box<dyn TypeLoader>>,
    #[serde(skip)]
    pub factories: BTreeMap<&'static str, Box<dyn TypeFactory>>,
}

impl Abi {
    pub fn new<T>(mut bytes: T, libraries: Option<Vec<Box<dyn Library>>>) -> Result<Self>
    where
        T: Read,
    {
        let libraries = match libraries {
            Some(mut libraries) => {
                let has_std = libraries
                    .iter()
                    .find(|lib| lib.name() == "std")
                    .map(|_| true)
                    .unwrap_or(false);
                if !has_std {
                    libraries.push(Box::<StdLibrary>::default());
                    libraries
                } else {
                    libraries
                }
            }
            None => {
                let libraries: Vec<Box<dyn Library>> = vec![Box::<StdLibrary>::default()];
                libraries
            }
        };

        let headers = bytes.read_abi_headers()?;
        let symbols = bytes.read_abi_symbols()?;
        let types = bytes.read_abi_types(&symbols)?;
        let functions = bytes.read_abi_functions(&symbols, &types)?;

        Ok(Self {
            headers,
            symbols,
            types,
            functions,
            libraries,
            loaders: Default::default(),
            factories: Default::default(),
        })
    }

    pub fn get_symbol_id(&self, value: &str) -> Option<u32> {
        self.symbols.id_by_name.get(value).copied()
    }

    pub fn get_symbol_by_id(&self, id: u32) -> AbiSymbol {
        AbiSymbol(&self.symbols[id])
    }

    pub fn get_symbol(&self, str: &str) -> Option<AbiSymbol> {
        self.symbols
            .id_by_name
            .get(str)
            .map(|id| self.get_symbol_by_id(*id))
    }

    pub fn get_type_by_fqn(&self, fqn: &str) -> Option<Rc<AbiType>> {
        let (module, name) = self.parse_fqn(fqn)?;
        self.types
            .iter()
            .find(|t| t.module == module && t.name == name)
            .cloned()
    }

    pub fn get_type_by_module_and_name(&self, module: &str, name: &str) -> Option<Rc<AbiType>> {
        let (module, name) = self.module_and_name(module, name)?;
        self.types
            .iter()
            .find(|t| t.module == module && t.name == name)
            .cloned()
    }

    pub fn get_fn_by_fqn(&self, fqn: &str) -> Option<&AbiFn> {
        let (module, name) = self.parse_fqn(fqn)?;
        self.functions
            .iter()
            .find(|f| f.module == module && f.name == name)
    }

    pub fn get_modvars(&self) -> Vec<ModVar> {
        let mut modvars = Vec::new();

        if let Some(root) = self.get_type_by_fqn("::$$$root") {
            if let Some(attrs) = root.attrs.as_deref() {
                for attr in attrs {
                    let (module, name) = self.symbols[attr.name]
                        .split_once('.')
                        .expect("module vars are supposed to be named '<module>.<name>'");

                    modvars.push(ModVar {
                        module: self.symbols.id_by_name.get(module).copied().unwrap(),
                        name: self.symbols.id_by_name.get(name).copied().unwrap(),
                        ty: attr.prog_type_offset.clone().into_inner(),
                        nullable: attr.nullable,
                    });
                }
            }
        }

        modvars
    }

    /// Splits a `"<module>::<name>"` fqn into the corresponding tuple `(<module:u32>, <name:u32>)`
    fn parse_fqn(&self, fqn: &str) -> Option<(u32, u32)> {
        let (module, name) = fqn.split_once("::")?;
        self.module_and_name(module, name)
    }

    /// Returns the corresponding tuple `(<module:u32>, <name:u32>)`
    fn module_and_name(&self, module: &str, name: &str) -> Option<(u32, u32)> {
        let module = self.symbols.id_by_name.get(module);
        let name = self.symbols.id_by_name.get(name);
        if let (Some(module), Some(name)) = (module, name) {
            Some((*module, *name))
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Abi")
            .field("headers", &self.headers)
            .field("symbols", &self.symbols)
            .field("types", &self.types)
            .field("functions", &self.functions)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AbiHeaders {
    pub headers: RequestHeaders,
    pub crc: u64,
}

impl AbiHeaders {
    pub fn from_bytes<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_abi_headers()
    }
}

impl AbiSerialize for AbiHeaders {
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        // AbiHeaders have no headers that precedes them
        self.write_raw_to(writer, abi)
    }

    #[inline]
    fn write_raw_to<W: Write>(&self, writer: &mut W, _abi: &Abi) -> Result<usize> {
        writer.write_u16::<LE>(self.headers.protocol)?;
        writer.write_u16::<LE>(self.headers.magic)?;
        writer.write_u32::<LE>(self.headers.version)?;
        Ok(8) // 2 + 2 + 4
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct RequestHeaders {
    pub protocol: u16,
    pub magic: u16,
    pub version: u32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AbiSymbol<'abi>(pub &'abi str);

impl AbiSerialize for AbiSymbol<'_> {
    #[inline]
    fn write_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        writer.write_u8(crate::primitive::STR_LIT)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    #[inline]
    fn write_raw_to<W: Write>(&self, writer: &mut W, abi: &Abi) -> Result<usize> {
        use crate::varint::VarintWrite;

        let id = abi
            .get_symbol_id(self.0)
            .ok_or_else(|| anyhow!("unknown symbol '{}'", self.0))?;

        let n = writer.write_vu32((id << 1) | 1)?;
        Ok(n)
    }
}

impl std::fmt::Display for AbiSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct AbiSymbols {
    pub symbols: Box<[Box<str>]>,
    id_by_name: HashMap<&'static str, u32>,
}

impl serde::Serialize for AbiSymbols {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.symbols.len()))?;
        for symb in self.symbols.iter() {
            seq.serialize_element(symb)?;
        }
        seq.end()
    }
}

impl AbiSymbols {
    pub fn from_bytes<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_abi_symbols()
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default, Clone)]
pub struct AbiTypes {
    types: Box<[Rc<AbiType>]>,
    pub core: CoreType,
}

impl serde::Serialize for AbiTypes {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.types.len()))?;
        for symb in self.types.iter() {
            seq.serialize_element(symb)?;
        }
        seq.end()
    }
}

impl AbiTypes {
    pub fn from_bytes<R: std::io::Read>(
        reader: &mut R,
        symbols: &AbiSymbols,
    ) -> std::io::Result<Self> {
        reader.read_abi_types(symbols)
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct CoreType {
    pub string: u32,
    pub char: u32,
    pub array: u32,
    pub map: u32,
    pub node: u32,
    pub node_list: u32,
    pub node_index: u32,
    pub node_time: u32,
    pub node_geo: u32,
}

#[derive(Debug, Clone)]
pub struct AbiFunctions {
    pub functions: Vec<AbiFn>,
    pub functions_by_id: HashMap<String, u32>,
}

impl AbiFunctions {
    pub fn from_bytes<R: std::io::Read>(
        reader: &mut R,
        symbols: &AbiSymbols,
        types: &AbiTypes,
    ) -> std::io::Result<Self> {
        reader.read_abi_functions(symbols, types)
    }
}

impl serde::Serialize for AbiFunctions {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.functions.len()))?;
        for symb in self.functions.iter() {
            seq.serialize_element(symb)?;
        }
        seq.end()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub struct AbiType {
    pub lib_name: u32,
    pub module: u32,
    pub name: u32,
    pub mapped_abi_type_offset: u32,
    #[serde(skip)]
    pub masked_abi_type_offset: u32,
    pub nullable_nb_bytes: u32,
    pub is_native: bool,
    pub is_abstract: bool,
    pub is_enum: bool,
    pub is_masked: bool,
    pub attrs: Option<Box<[AbiAttr]>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum LazyAbiType {
    #[serde(rename = "prog_type_offset")]
    Offset(u32),
    #[serde(
        rename = "type",
        serialize_with = "crate::serde_utils::serialize_type_as_fqn"
    )]
    Ref(Rc<AbiType>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub struct AbiAttr {
    pub name: u32,
    pub abi_type: u32,
    #[serde(flatten)]
    pub prog_type_offset: RefCell<LazyAbiType>,
    #[serde(skip)]
    pub mapped_any_offset: u32,
    pub mapped_att_offset: u32,
    pub sbi_type: u8,
    pub nullable: bool,
    pub mapped: bool,
}

impl std::fmt::Display for AbiAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attr_type = self.prog_type_offset.borrow();
        match &*attr_type {
            LazyAbiType::Offset(offset) => write!(f, "{}: Type#{offset}", self.name),
            LazyAbiType::Ref(ty) => write!(f, "{}: {}::{}", self.name, ty.module, ty.name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct AbiFn {
    pub module: u32,
    pub r#type: Option<u32>,
    pub name: u32,
    pub lib_name: u32,
    pub params: Vec<AbiParam>,
    #[serde(serialize_with = "crate::serde_utils::serialize_type_as_fqn")]
    pub return_type: Rc<AbiType>,
    pub return_nullable: bool,
    pub is_task: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct AbiParam {
    pub name: u32,
    #[serde(serialize_with = "crate::serde_utils::serialize_type_as_fqn")]
    pub r#type: Rc<AbiType>,
    pub nullable: bool,
}

pub trait AbiHeadersRead {
    fn read_abi_headers(&mut self) -> std::io::Result<AbiHeaders>;
}

impl<T: Read> AbiHeadersRead for T {
    fn read_abi_headers(&mut self) -> std::io::Result<AbiHeaders> {
        let headers = self.read_request_headers()?;
        let crc = self.read_u64::<LE>()?;
        Ok(AbiHeaders { headers, crc })
    }
}

pub trait RequestHeadersRead {
    fn read_request_headers(&mut self) -> std::io::Result<RequestHeaders>;
}

impl<T: Read> RequestHeadersRead for T {
    fn read_request_headers(&mut self) -> std::io::Result<RequestHeaders> {
        let protocol = self.read_u16::<LE>()?;
        let magic = self.read_u16::<LE>()?;
        let version = self.read_u32::<LE>()?;
        Ok(RequestHeaders {
            protocol,
            magic,
            version,
        })
    }
}

pub trait AbiSymbolsRead {
    fn read_abi_symbols(&mut self) -> std::io::Result<AbiSymbols>;
}

impl<T: Read> AbiSymbolsRead for T {
    fn read_abi_symbols(&mut self) -> std::io::Result<AbiSymbols> {
        let _symbols_size = self.read_u64::<LE>()?;
        let nb_symbols = self.read_u32::<LE>()? as usize;
        let mut symbols: Vec<Box<str>> = Vec::with_capacity(nb_symbols + 1);
        symbols.push("".into());
        for _ in 0..nb_symbols {
            let len: u32 = self.read_vu32()?;
            let mut buf = String::with_capacity(len as usize);
            self.take(len as u64).read_to_string(&mut buf)?;
            let buf_ptr = buf.as_ptr();
            let string: Box<str> = buf.into();
            debug_assert_eq!(buf_ptr, string.as_ptr(), "unexpected re-alloc");
            symbols.push(string);
        }

        let mut id_by_name: HashMap<&'static str, u32> = HashMap::with_capacity(nb_symbols + 1);

        let symbols = symbols.into_boxed_slice();
        for (i, str) in symbols.iter().enumerate() {
            // SAFETY:
            // We boxed the symbols vec and we know that for the lifetime of an ABI
            // we never mutate `symbols` again. Ever.
            // We also don't give direct access to `symbols`, nor `id_by_name`.
            // Therefore, it is fine to lie to the compiler and tell it that
            // those references are static because they are never gonna be moved.
            //
            // To make it crystal clear, we only impl std::ops::Index for AbiSymbols
            // so we don't even leak those `&'static str`, we give references to the boxed slice of boxed str
            let key: &'static str = unsafe { std::mem::transmute(&**str) };
            id_by_name.insert(key, i as u32);
        }

        Ok(AbiSymbols {
            symbols,
            id_by_name,
        })
    }
}

pub trait AbiTypesRead {
    fn read_abi_types(&mut self, symbols: &AbiSymbols) -> std::io::Result<AbiTypes>;
}

impl<T: Read> AbiTypesRead for T {
    fn read_abi_types(&mut self, symbols: &AbiSymbols) -> std::io::Result<AbiTypes> {
        // types
        let _types_bin_size = self.read_u64::<LE>()?; // types binary size
        let nb_types = self.read_u32::<LE>()?;
        let _nb_attrs = self.read_u32::<LE>()?;

        let mut types: Vec<Rc<AbiType>> = Vec::with_capacity(nb_types as usize);
        let mut core = CoreType::default();

        for i in 0..nb_types {
            // parse types
            let module: u32 = self.read_vu32()?;
            let name: u32 = self.read_vu32()?;
            let lib_name: u32 = self.read_vu32()?;
            let attributes_len: u32 = self.read_vu32()?;
            let _attributes_offset: u32 = self.read_vu32()?;
            let _mapped_prog_type_offset: u32 = self.read_vu32()?;
            let mapped_abi_type_offset: u32 = self.read_vu32()?;
            let masked_abi_type_offset: u32 = self.read_vu32()?;
            let nullable_nb_bytes: u32 = self.read_vu32()?;
            let flags = self.read_u8()?;
            let is_native = (flags & 1) != 0;
            let is_abstract = (flags & (1 << 1)) != 0;
            let is_enum = (flags & (1 << 2)) != 0;
            let is_masked = (flags & (1 << 3)) != 0;

            let attrs = if attributes_len > 0 {
                let mut attrs = Vec::with_capacity(attributes_len as usize);
                for _ in 0..attributes_len {
                    // parse attribute
                    let name: u32 = self.read_vu32()?;
                    let abi_type: u32 = self.read_vu32()?;
                    let prog_type_offset: u32 = self.read_vu32()?;
                    let mapped_any_offset: u32 = self.read_vu32()?;
                    let mapped_att_offset: u32 = self.read_vu32()?;
                    let sbi_type = self.read_u8()?;
                    let flags = self.read_u8()?;
                    let nullable = (flags & 1) != 0;
                    let mapped = (flags & (1 << 1)) != 0;

                    let prog_type_offset = match types.get(prog_type_offset as usize) {
                        Some(ty) => RefCell::new(LazyAbiType::Ref(ty.clone())),
                        None => RefCell::new(LazyAbiType::Offset(prog_type_offset)),
                    };

                    attrs.push(AbiAttr {
                        name,
                        abi_type,
                        prog_type_offset,
                        mapped_any_offset,
                        mapped_att_offset,
                        sbi_type,
                        nullable,
                        mapped,
                    });
                }
                Some(attrs.into_boxed_slice())
            } else {
                None
            };

            if &symbols[module] == "core" {
                match &symbols[name] {
                    "String" => core.string = i,
                    "Array" => core.array = i,
                    "Map" => core.map = i,
                    _ => (),
                }
            }

            let ty = Rc::new(AbiType {
                module,
                name,
                lib_name,
                mapped_abi_type_offset,
                masked_abi_type_offset,
                nullable_nb_bytes,
                is_native,
                is_abstract,
                is_enum,
                is_masked,
                attrs,
            });

            types.push(ty);
        }

        for ty in &types {
            if let Some(attrs) = &ty.attrs {
                for attr in attrs.iter() {
                    let mut ty = attr.prog_type_offset.borrow_mut();
                    if let LazyAbiType::Offset(offset) = *ty {
                        *ty = LazyAbiType::Ref(types[offset as usize].clone());
                    }
                }
            }
        }

        Ok(AbiTypes {
            types: Box::from(types),
            core,
        })
    }
}

impl AbiTypes {
    pub fn get(&self, id: u32) -> Option<Rc<AbiType>> {
        self.types.get(id as usize).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModVar {
    module: u32,
    name: u32,
    #[serde(rename = "type", flatten)]
    ty: LazyAbiType,
    nullable: bool,
}

impl IntoIterator for AbiFunctions {
    type Item = AbiFn;
    type IntoIter = std::vec::IntoIter<AbiFn>;

    fn into_iter(self) -> Self::IntoIter {
        self.functions.into_iter()
    }
}

impl std::ops::Deref for AbiSymbols {
    type Target = [Box<str>];

    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}

impl std::ops::Deref for AbiTypes {
    type Target = [Rc<AbiType>];

    fn deref(&self) -> &Self::Target {
        &self.types
    }
}

impl std::ops::Deref for AbiFunctions {
    type Target = Vec<AbiFn>;

    fn deref(&self) -> &Self::Target {
        &self.functions
    }
}

impl std::ops::DerefMut for AbiFunctions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.functions
    }
}

pub trait AbiFunctionsRead {
    fn read_abi_functions(
        &mut self,
        symbols: &AbiSymbols,
        types: &AbiTypes,
    ) -> std::io::Result<AbiFunctions>;
}

impl<T: Read> AbiFunctionsRead for T {
    fn read_abi_functions(
        &mut self,
        _symbols: &AbiSymbols,
        types: &AbiTypes,
    ) -> std::io::Result<AbiFunctions> {
        let _functions_bin_size = self.read_u64::<LE>()?;
        let functions_len = self.read_u32::<LE>()?;

        let mut functions = Vec::with_capacity(functions_len as usize);
        let mut functions_by_id = HashMap::with_capacity(functions_len as usize);

        for fn_idx in 0..functions_len {
            let module: u32 = self.read_vu32()?;
            let ty: u32 = self.read_vu32()?;
            let name: u32 = self.read_vu32()?;
            let lib_name: u32 = self.read_vu32()?;
            let param_nb: u32 = self.read_vu32()?;
            let mut params = Vec::with_capacity(param_nb as usize);
            for _ in 0..param_nb {
                let param_nullable = self.read_u8()? != 0;
                let param_type: u32 = self.read_vu32()?;
                let param_symbol: u32 = self.read_vu32()?;
                params.push(AbiParam {
                    name: param_symbol,
                    nullable: param_nullable,
                    r#type: types[param_type].clone(),
                });
            }
            let return_type: u32 = self.read_vu32()?;
            let flags = self.read_u8()?;
            let return_nullable = (flags & 1) != 0;
            let is_task = (flags & (1 << 1)) != 0;

            let function = AbiFn {
                lib_name,
                module,
                r#type: if ty == 0 { None } else { Some(ty) },
                name,
                is_task,
                return_nullable,
                return_type: types[return_type].clone(),
                params,
            };
            functions_by_id.insert(function.fqn(), fn_idx);
            functions.push(function);
        }

        Ok(AbiFunctions {
            functions,
            functions_by_id,
        })
    }
}

impl AbiType {
    pub fn fqn(&self) -> String {
        format!("{}::{}", self.module, self.name)
    }

    pub fn named_fqn(&self, abi: &Abi) -> String {
        let module = &abi.symbols[self.module];
        let name = &abi.symbols[self.name];
        format!("{module}::{name}")
    }
}

impl AbiFn {
    pub fn fqn(&self) -> String {
        match self.r#type {
            Some(ty) => format!("{}::{ty}::{}", self.module, self.name),
            None => format!("{}::{}", self.module, self.name),
        }
    }
}

impl std::ops::Index<u32> for AbiTypes {
    type Output = Rc<AbiType>;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.types[index as usize]
    }
}

impl std::ops::Index<usize> for AbiTypes {
    type Output = Rc<AbiType>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.types[index]
    }
}

impl AbiSymbols {
    #[inline]
    pub fn get(&self, symbol: &str) -> Option<u32> {
        self.id_by_name.get(symbol).copied()
    }
}

impl std::ops::Index<u32> for AbiSymbols {
    type Output = str;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.symbols[index as usize]
    }
}

impl std::ops::Index<usize> for AbiSymbols {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index]
    }
}
