use std::ffi::CString;

use crate::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Machine(pub *mut gc_machine_t);

impl Machine {
    pub fn get_buffer(&self) -> Buffer {
        Buffer::from(unsafe { gc_machine__get_buffer(self.0) })
    }

    pub fn set_error(&mut self, message: &str) {
        let c_str =
            CString::new(message).expect("set_error 'message' must not contain any 0 bytes");
        unsafe { gc_machine__set_runtime_error(self.0, c_str.as_ptr()) }
    }

    pub fn get_string_param(&self, offset: u32) -> GStr {
        let slot = unsafe { gc_machine__get_param(self.0, offset) };
        GStr::from(unsafe { slot.object as *mut gc_core_string_t })
    }

    pub fn get_opt_string_param(&self, offset: u32) -> Option<GStr> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_string_param(offset))
        }
    }

    #[inline]
    pub fn get_this(&self) -> GObj {
        GObj::from(unsafe { gc_machine__this(self.0).object })
    }

    #[inline(always)]
    pub fn get_param_slot(&self, offset: u32) -> (gc_slot_t, gc_type_t) {
        unsafe {
            (
                gc_machine__get_param(self.0, offset),
                gc_machine__get_param_type(self.0, offset),
            )
        }
    }

    #[inline]
    pub fn get_param(&self, offset: u32) -> Value {
        self.get_param_slot(offset).into()
    }

    #[inline]
    pub fn get_opt_param(&self, offset: u32) -> Option<Value> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_param_slot(offset).into())
        }
    }

    #[inline]
    pub fn get_int_param(&self, offset: u32) -> i64 {
        unsafe { gc_machine__get_param(self.0, offset).i64 }
    }

    #[inline]
    pub fn get_opt_int_param(&self, offset: u32) -> Option<i64> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_int_param(offset))
        }
    }

    #[inline]
    pub fn get_float_param(&self, offset: u32) -> f64 {
        unsafe { gc_machine__get_param(self.0, offset).f64 }
    }

    #[inline]
    pub fn get_opt_float_param(&self, offset: u32) -> Option<f64> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_float_param(offset))
        }
    }

    #[inline]
    pub fn get_bool_param(&self, offset: u32) -> bool {
        unsafe { gc_machine__get_param(self.0, offset).b }
    }

    #[inline]
    pub fn get_opt_bool_param(&self, offset: u32) -> Option<bool> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_bool_param(offset))
        }
    }

    #[inline]
    pub fn get_char_param(&self, offset: u32) -> char {
        unsafe { std::char::from_u32_unchecked(gc_machine__get_param(self.0, offset).u32) }
    }

    #[inline]
    pub fn get_opt_char_param(&self, offset: u32) -> Option<char> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_char_param(offset))
        }
    }

    #[inline]
    pub fn get_tu2d_param(&self, offset: u32) -> (u32, u32) {
        let [a, b] = unsafe { gc_machine__get_param(self.0, offset).tuple.u2 };
        (a, b)
    }

    #[inline]
    pub fn get_opt_tu32_param(&self, offset: u32) -> Option<(u32, u32)> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_tu2d_param(offset))
        }
    }

    #[inline]
    pub fn get_obj_param(&self, offset: u32) -> GObj {
        unsafe { gc_machine__get_param(self.0, offset).object }.into()
    }

    #[inline]
    pub fn get_opt_obj_param(&self, offset: u32) -> Option<GObj> {
        if self.is_null(offset) {
            None
        } else {
            Some(self.get_obj_param(offset))
        }
    }

    #[inline]
    pub fn get_param_type(&self, offset: u32) -> gc_type_t {
        unsafe { gc_machine__get_param_type(self.0, offset) }
    }

    #[inline]
    pub fn create_obj(&self, type_id: u32) -> GObject {
        GObject::new(
            unsafe { gc_machine__create_object(self.0, type_id) },
            self.0,
        )
    }

    #[inline]
    pub fn set_result(&mut self, value: impl AsSlot) {
        let (slot, slot_type) = value.as_slot();
        unsafe { gc_machine__set_result(self.0, slot, slot_type) }
    }

    #[inline]
    pub fn set_result_int(&mut self, value: i64) {
        unsafe { gc_machine__set_result(self.0, value.into(), gc_type_int) }
    }

    #[inline]
    pub fn set_result_float(&mut self, value: f64) {
        unsafe { gc_machine__set_result(self.0, value.into(), gc_type_float) }
    }

    #[inline]
    pub fn set_result_bool(&mut self, value: bool) {
        unsafe { gc_machine__set_result(self.0, value.into(), gc_type_bool) }
    }

    #[inline]
    pub fn set_result_null(&mut self) {
        unsafe { gc_machine__set_result(self.0, gc_slot_t::default(), gc_type_null) }
    }

    #[inline]
    pub fn set_result_obj(&mut self, value: *mut gc_object_t) {
        unsafe { gc_machine__set_result(self.0, value.into(), gc_type_object) }
    }

    #[inline]
    fn is_null(&self, offset: u32) -> bool {
        unsafe { gc_machine__get_param_type(self.0, offset) == gc_type_null }
    }
}
