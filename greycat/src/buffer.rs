use greycat_sys::*;

use crate::{gstring::GString, Machine, Value};

/// Borrowed `gc_buffer_t` from the current `gc_machine_t`
#[repr(transparent)]
pub struct Buffer(*mut gc_buffer_t);

impl From<*mut gc_buffer_t> for Buffer {
    #[inline(always)]
    fn from(value: *mut gc_buffer_t) -> Self {
        Self(value)
    }
}

impl Buffer {
    pub fn clear(&self) {
        unsafe { gc_buffer__clear(self.0) }
    }

    pub fn prepare(&self, needed: u32) {
        unsafe { gc_buffer__prepare(self.0, needed) }
    }

    pub fn len(&self) -> u32 {
        unsafe { gc_buffer__size(self.0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_value(&self, value: &Value, ctx: &Machine) {
        let (slot, ty) = value.into();
        unsafe { gc_buffer__add_slot(self.0, slot, ty, ctx.0) }
    }

    pub fn push(&self, c: char) {
        unsafe {
            gc_buffer__add_char(self.0, c as i8);
        }
    }

    pub fn push_str(&self, str: impl AsRef<str>) {
        let str = str.as_ref();
        let len = str.len();
        let ptr = str.as_ptr();
        unsafe { gc_buffer__add_str(self.0, ptr as _, len as _) }
    }

    pub fn to_gstr(&self, ctx: &Machine) -> GString {
        let ptr = unsafe { gc_core_string__create_from_buffer(self.0) };
        GString { ptr, ctx: ctx.0 }
    }
}
