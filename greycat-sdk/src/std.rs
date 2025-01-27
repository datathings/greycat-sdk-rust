use std::collections::BTreeMap;
use std::rc::Rc;

use anyhow::Result;

use crate::abi::{Abi, AbiType};
use crate::library::*;

#[derive(Default, Clone)]
pub struct StdLibrary {
    _mapped: Vec<Rc<AbiType>>,
}

impl Library for StdLibrary {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "std"
    }

    fn configure(
        &self,
        _loaders: &mut BTreeMap<&'static str, Box<dyn TypeLoader>>,
        _factories: &mut BTreeMap<&'static str, Box<dyn TypeFactory>>,
    ) -> Result<()> {
        // loaders.insert("core::String", Box::new(TypeLoader::<GcObject>::load));
        Ok(())
    }

    fn init(&mut self, _abi: &Abi) -> Result<()> {
        todo!()
    }
}
