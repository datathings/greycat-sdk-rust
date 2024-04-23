use greycat_sys::*;

use crate::{AsSlot, Value};

/// Borrowed `gc_core_string_t`
#[repr(transparent)]
pub struct GStr(*mut gc_core_string_t);

impl From<*mut gc_core_string_t> for GStr {
    #[inline(always)]
    fn from(value: *mut gc_core_string_t) -> Self {
        Self(value)
    }
}

impl AsRef<str> for &GStr {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl GStr {
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.as_ptr(), self.len());
            std::str::from_utf8_unchecked(slice)
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        unsafe { gc_core_string__buffer(self.0) as _ }
    }

    pub fn len(&self) -> usize {
        unsafe { gc_core_string__size(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<&GStr> for Value {
    #[inline(always)]
    fn from(val: &GStr) -> Self {
        Value::from(val.0)
    }
}

impl AsSlot for GStr {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        (self.0.into(), gc_type_object)
    }
}
