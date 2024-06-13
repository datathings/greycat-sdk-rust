#![allow(clippy::not_unsafe_ptr_arg_deref)]
use crate::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Program(*mut gc_program_t);

impl From<*mut gc_program_t> for Program {
    #[inline(always)]
    fn from(value: *mut gc_program_t) -> Self {
        Self(value)
    }
}

impl Program {
    #[inline]
    pub fn resolve_symbol_id(&self, symbol: &str) -> u32 {
        unsafe {
            gc_program__resolve_symbol(self.0, symbol.as_ptr() as *const _, symbol.len() as _)
        }
    }

    #[inline]
    pub fn resolve_module_id(&self, name: &str) -> u32 {
        unsafe {
            let id = gc_program__resolve_symbol(self.0, name.as_ptr() as *const _, name.len() as _);
            gc_program__resolve_module(self.0, id)
        }
    }

    #[inline]
    pub fn resolve_type_id(&self, mod_id: u32, type_name: &str) -> u32 {
        if mod_id == 0 {
            return 0;
        }
        unsafe {
            let type_id = gc_program__resolve_symbol(
                self.0,
                type_name.as_ptr() as *const _,
                type_name.len() as _,
            );
            gc_program__resolve_type(self.0, mod_id, type_id)
        }
    }

    pub fn link_type<const F: usize>(&self, mod_name: &str, ty: Type<'_, F>) -> bool {
        let mod_id = self.resolve_module_id(mod_name);
        if mod_id == 0 {
            return false;
        }
        let type_id = self.resolve_type_id(mod_id, ty.name);
        if type_id == 0 {
            return false;
        }

        let fn_offsets: &mut [u32; F] = &mut [0; F];
        for (i, offset) in fn_offsets.iter_mut().enumerate() {
            *offset = i as u32;
        }

        unsafe {
            gc_program__link_type(
                self.0,
                type_id,
                ty.foreach_slots,
                ty.load,
                ty.save,
                ty.to_string,
                ty.functions.as_ptr(),
                fn_offsets.as_mut_ptr(),
                ty.functions.len() as _,
            )
        };

        true
    }

    pub fn link_mod<const F: usize>(&self, module: &Module<'_, F>) -> bool {
        let mod_id = self.resolve_module_id(module.name);
        if mod_id == 0 {
            return false;
        }

        let fn_offsets: &mut [u32; F] = &mut [0; F];
        for (i, offset) in fn_offsets.iter_mut().enumerate() {
            *offset = i as u32;
        }

        unsafe {
            gc_program__link_mod(
                self.0,
                mod_id,
                module.functions.as_ptr(),
                fn_offsets.as_mut_ptr(),
                module.functions.len() as _,
            )
        };

        true
    }
}

pub struct Module<'a, const F: usize> {
    /// The name of the module
    pub name: &'a str,
    /// The list of native function pointers to register
    pub functions: [gc_program_function_body_t; F],
    // /// An optional function pointer called once when the module is loaded
    // pub start: gc_program_module_handle_t,
    // /// An optional function pointer called once when the module is unloaded
    // pub stop: gc_program_module_handle_t,
}

pub struct Type<'a, const F: usize> {
    pub name: &'a str,
    pub foreach_slots: gc_object_type_foreach_slots_t,
    pub load: gc_object_type_load_t,
    pub save: gc_object_type_save_t,
    pub create: gc_object_type_create_t,
    pub finalize: gc_object_type_finalize_t,
    pub to_string: gc_object_type_to_string_t,
    pub functions: [gc_program_function_body_t; F],
}
