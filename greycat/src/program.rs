use greycat_sys::*;

use crate::{
    ffi::FromPtr as _,
    machine::GcMachine,
    object::{AsGcObject, GcObjectRef}, types::GcString,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GcProgram(pub *const gc_program_t);

impl GcProgram {
    #[inline(always)]
    pub fn get_symbol(&self, offset: u32) -> GcSymbol {
        GcSymbol(unsafe { gc_program__get_symbol(self.0, offset) })
    }

    #[inline(always)]
    pub fn get_symbol_id(&self, symbol: &GcSymbol) -> GcSymbolId {
        GcSymbolId(unsafe { gc_program__get_symbol_off(symbol.0) })
    }

    #[inline(always)]
    pub fn resolve_symbol(&self, text: &str) -> GcSymbolId {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        assert!(id != 0, "unable to resolve symbol '{text}'");
        GcSymbolId(id)
    }

    #[inline(always)]
    pub fn resolve_symbol_opt(&self, text: &str) -> Option<GcSymbolId> {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        if id == 0 {
            None
        } else {
            Some(GcSymbolId(id))
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GcProgramMut(pub *mut gc_program_t);

impl GcProgramMut {
    #[inline(always)]
    pub fn get_symbol(&self, offset: u32) -> GcSymbol {
        GcSymbol(unsafe { gc_program__get_symbol(self.0, offset) })
    }

    #[inline(always)]
    pub fn get_symbol_id(&self, symbol: &GcSymbol) -> GcSymbolId {
        GcSymbolId(unsafe { gc_program__get_symbol_off(symbol.0) })
    }

    #[inline(always)]
    pub fn resolve_symbol(&self, text: &str) -> GcSymbolId {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        assert!(id != 0, "unable to resolve symbol '{text}'");
        GcSymbolId(id)
    }

    #[inline(always)]
    pub fn resolve_type(&self, module: GcModuleId, type_name: &str) -> GcTypeId {
        let type_name_off = self.resolve_symbol(type_name);
        let id = unsafe { gc_program__resolve_type(self.0, module.0, type_name_off.0) };
        assert!(
            id != 0,
            "unable to resolve type '{type_name}' in module {module:?}"
        );
        GcTypeId(id)
    }

    #[inline(always)]
    pub fn resolve_module(&self, name: &str) -> GcModuleId {
        let mod_name_offset = self.resolve_symbol(name);
        let id = unsafe { gc_program__resolve_module(self.0, mod_name_offset.0) };
        assert!(id != 0, "unable to resolve module '{name}'");
        GcModuleId(id)
    }

    #[inline(always)]
    pub fn link_mod_fn(&self, module: GcModuleId, fn_name: &str, fn_impl: GcMachineFn) {
        let fn_name = self.resolve_symbol(fn_name);
        assert!(
            unsafe { gc_program__link_mod_fn(self.0, module.0, Some(fn_impl), fn_name.0) },
            "unable to link module function {fn_name:?} for module {module:?}"
        )
    }

    #[inline(always)]
    pub fn link_type_fn(&self, type_id: GcTypeId, fn_name: &str, function: GcMachineFn) {
        let fn_id = self.resolve_symbol(fn_name);
        assert!(
            unsafe { gc_program__link_type_fn(self.0, type_id.0, Some(function), fn_id.0) },
            "unable to link type function {fn_name} from type {type_id:?}"
        )
    }

    #[inline(always)]
    pub fn configure_type(&self, type_id: GcTypeId, bytes_size: usize, function: Option<GcObjectFinalizeFn>) {
        unsafe { gc_program_type__configure(self.0, type_id.0, bytes_size as _, function) }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GcModuleId(u32);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GcSymbolId(u32);

impl GcSymbolId {
    #[inline(always)]
    pub fn as_string(&self, ctx: GcMachine) -> GcString {
        let ptr = unsafe { gc_program__get_symbol(ctx.get_program().0, self.0) };
        GcString::from_ptr(ptr)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GcTypeId(u32);

#[repr(transparent)]
pub struct GcSymbol(pub *mut gc_program_symbol_t);

impl GcSymbol {
    #[inline(always)]
    pub fn id(&self) -> GcSymbolId {
        GcSymbolId(unsafe { gc_program__get_symbol_off(self.0) })
    }
}

impl AsGcObject for GcSymbol {
    #[inline(always)]
    fn as_object(&self) -> GcObjectRef {
        GcObjectRef(self.0 as _)
    }
}

pub type GcMachineFn = unsafe extern "C" fn(ctx: *mut gc_machine_t);
pub type GcObjectFinalizeFn = unsafe extern "C" fn(this: *mut gc_object_t, ctx: *mut gc_machine_t);
