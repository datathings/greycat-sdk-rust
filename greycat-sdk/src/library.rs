use std::{collections::BTreeMap, rc::Rc};

use anyhow::Result;

use crate::abi::{Abi, AbiType};
use crate::value::Value;

pub trait TypeLoader {
    fn load(&mut self, ty: Rc<AbiType>, abi: &Abi) -> Result<Value>;
}

pub trait TypeFactory {
    fn create(ty: Rc<AbiType>, attrs: Option<Box<[Value]>>) -> Result<Self>
    where
        Self: Sized;
}

pub trait Library {
    fn name(&self) -> &'static str;

    fn configure(
        &self,
        loaders: &mut BTreeMap<&'static str, Box<dyn TypeLoader>>,
        factories: &mut BTreeMap<&'static str, Box<dyn TypeFactory>>,
    ) -> Result<()>;

    fn init(&mut self, abi: &Abi) -> Result<()>;
}
