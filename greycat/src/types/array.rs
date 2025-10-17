use crate::ffi::FromPtr as _;
use crate::machine::GcMachine;
use crate::types::GcString;
use crate::value::AsGcValue;
use greycat_sys::*;

#[repr(transparent)]
pub struct GcArray(*mut gc_core_array_t);

impl GcArray {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(ctx: GcMachine) -> Self {
        let ptr = unsafe {
            // TODO those should be cached
            let prog = ctx.get_program();
            let mod_name = "core";
            let mod_name_offset =
                gc_program__resolve_symbol(prog.0, mod_name.as_ptr() as _, mod_name.len() as _);
            let mod_offset = gc_program__resolve_module(prog.0, mod_name_offset);
            let type_name = "Array<core::any>";
            let type_name_off =
                gc_program__resolve_symbol(prog.0, type_name.as_ptr() as _, type_name.len() as _);
            let object_type_code = gc_program__resolve_type(prog.0, mod_offset, type_name_off);
            ctx.create_object(object_type_code) as _
        };
        Self(ptr)
    }

    pub fn add(&mut self, elem: impl AsGcValue, ctx: GcMachine) -> bool {
        let (value, value_type) = elem.as_value();
        self.add_slot(value, value_type, ctx.0)
    }

    pub fn add_str(&mut self, elem: &str, ctx: GcMachine) -> bool {
        let prog = ctx.get_program();
        let value = match prog.resolve_symbol_opt(elem) {
            Some(symb) => symb.as_string(ctx),
            None => {
                let ptr =
                    unsafe { gc_core_string__create_from(elem.as_ptr() as _, elem.len() as u64) };
                GcString::from_ptr(ptr)
            }
        };
        self.add(value, ctx)
    }

    pub fn set(&mut self, offset: u32, elem: impl AsGcValue, ctx: GcMachine) -> bool {
        let (value, value_type) = elem.as_value();
        self.set_slot(offset, value, value_type, ctx.0)
    }

    pub fn get(&mut self, offset: u32) -> Option<(gc_slot, gc_type)> {
        let mut slot = gc_slot::null();
        let mut slot_type = gc_type_null;
        unsafe {
            if gc_core_array__get_slot(self.0, offset, &mut slot, &mut slot_type) {
                if slot_type == gc_type_null {
                    None
                } else {
                    Some((slot, slot_type))
                }
            } else {
                None
            }
        }
    }

    pub fn size(&mut self) -> u32 {
        unsafe { gc_core_array__size(self.0) }
    }

    fn add_slot(
        &mut self,
        value: gc_slot_t,
        value_type: gc_type_t,
        ctx: *mut gc_machine_t,
    ) -> bool {
        let res = unsafe { gc_core_array__add_slot(self.0, value, value_type, ctx) };
        if res && value_type == gc_type_object {
            unsafe {
                gc_object__un_mark(value.__1.object, ctx);
            }
        }
        res
    }

    fn set_slot(
        &mut self,
        offset: u32,
        value: gc_slot_t,
        value_type: gc_type_t,
        ctx: *mut gc_machine_t,
    ) -> bool {
        let res = unsafe { gc_core_array__set_slot(self.0, offset, value, value_type, ctx) };
        if res && value_type == gc_type_object {
            unsafe {
                gc_object__un_mark(value.__1.object, ctx);
            }
        }
        res
    }
}

impl AsGcValue for GcArray {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (gc_slot::object(self.0 as _), gc_type_object)
    }
}

impl From<*mut gc_core_array_t> for GcArray {
    #[inline(always)]
    fn from(value: *mut gc_core_array_t) -> Self {
        Self(value)
    }
}
