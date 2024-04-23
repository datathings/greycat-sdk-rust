use greycat_sys::*;

use crate::{GStr, AsSlot, Machine, Value};

#[repr(C)]
/// Owned `gc_core_string_t` that will unmark itself when dropped
pub struct GString {
    pub(crate) ptr: *mut gc_core_string_t,
    pub(crate) ctx: *mut gc_machine_t,
}

impl AsRef<str> for &GString {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for GString {
    type Target = GStr;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr as *const GStr) }
    }
}

impl GString {
    /// Creates an owned `GString` from the given `&str`
    pub fn from_str(str: &str, ctx: &Machine) -> Self {
        let len = str.len() as u64;
        let str = str.as_ptr() as *const i8;
        Self {
            ptr: unsafe { gc_core_string__create_from(str, len) },
            ctx: ctx.0,
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.as_ptr(), self.len());
            std::str::from_utf8_unchecked(slice)
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        unsafe { gc_core_string__buffer(self.ptr) as _ }
    }

    pub fn len(&self) -> usize {
        unsafe { gc_core_string__size(self.ptr) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<&GString> for Value {
    #[inline(always)]
    fn from(val: &GString) -> Self {
        Value::from(val.ptr)
    }
}

impl AsSlot for GString {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        (self.ptr.into(), gc_type_object)
    }
}

impl Drop for GString {
    fn drop(&mut self) {
        unsafe { gc_object__un_mark(self.ptr as _, self.ctx) }
    }
}
