use std::ptr::NonNull;
use greycat_sys::*;
use crate::object::{FromPtr, AsPtr, AsPtrMut};

#[repr(transparent)]
pub struct GcBuffer(NonNull<gc_buffer_t>);

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

impl AsPtr for GcBuffer {
    fn as_ptr(&self) -> *const gc_object_t {
        self.0.as_ptr() as _
    }
}

impl AsPtrMut for GcBuffer {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t {
        self.0.as_ptr() as _
    }
}

impl FromPtr<gc_object_t> for GcBuffer {
    unsafe fn from_ptr(ptr: *mut gc_object_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr as _) })
    }
}

impl FromPtr<gc_buffer_t> for GcBuffer {
    unsafe fn from_ptr(ptr: *mut gc_buffer_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }
}