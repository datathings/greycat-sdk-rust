use std::ptr::NonNull;

use crate::machine::GcMachine;
use crate::value::AsGcValue;
use crate::{AsPtr, AsPtrMut, FromPtr};
use greycat_sys::*;

#[repr(transparent)]
pub struct GcArray(NonNull<gc_core_array_t>);

impl GcArray {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(ctx: GcMachine) -> Self {
        let prog = ctx.get_program();
        let array_core_any = prog.resolve_type("core", "Array<core::any>").unwrap();
        Self(unsafe { NonNull::new_unchecked(ctx.create_object(array_core_any)) })
    }

    pub fn add(&mut self, elem: impl AsGcValue, ctx: GcMachine) -> bool {
        let (value, value_type) = elem.to_value();
        self.add_slot(value, value_type, ctx.0)
    }

    pub fn set(&mut self, offset: u32, elem: impl AsGcValue, ctx: GcMachine) -> bool {
        let (value, value_type) = elem.to_value();
        self.set_slot(offset, value, value_type, ctx.0)
    }

    pub fn get(&mut self, offset: u32) -> Option<(gc_slot, gc_type)> {
        let mut slot = gc_slot::null();
        let mut slot_type = gc_type_null;
        unsafe {
            if gc_core_array__get_slot(self.0.as_ptr(), offset, &mut slot, &mut slot_type) {
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
        unsafe { gc_core_array__size(self.0.as_ptr()) }
    }

    fn add_slot(
        &mut self,
        value: gc_slot_t,
        value_type: gc_type_t,
        ctx: *mut gc_machine_t,
    ) -> bool {
        let res = unsafe { gc_core_array__add_slot(self.0.as_ptr(), value, value_type, ctx) };
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
        let res =
            unsafe { gc_core_array__set_slot(self.0.as_ptr(), offset, value, value_type, ctx) };
        if res && value_type == gc_type_object {
            unsafe {
                gc_object__un_mark(value.__1.object, ctx);
            }
        }
        res
    }
}

impl AsPtr for GcArray {
    fn as_ptr(&self) -> *const gc_object_t {
        self.0.as_ptr() as _
    }
}

impl AsPtrMut for GcArray {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t {
        self.0.as_ptr() as _
    }
}

impl FromPtr<gc_object_t> for GcArray {
    unsafe fn from_ptr(ptr: *mut gc_object_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr as _) })
    }
}
