use std::ptr::NonNull;

use greycat_sys::*;

use crate::ffi::FromPtr;

pub struct GcBuffer(NonNull<gc_buffer_t>);

impl FromPtr for GcBuffer {
    type CType = gc_buffer_t;

    fn from_ptr(ptr: *mut Self::CType) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }
}

impl GcBuffer {
    #[inline(always)]
    pub fn clear(&mut self) -> &mut Self {
        unsafe { gc_buffer__clear(self.0.as_ptr()) }
        self
    }

    #[inline(always)]
    pub fn push_str(&mut self, str: &str) -> &mut Self {
        unsafe { gc_buffer__add_str(self.0.as_ptr(), str.as_ptr() as *const _, str.len() as u32) }
        self
    }
}
