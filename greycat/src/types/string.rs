use crate::object::{AsPtr, AsPtrMut, WrapPtr};
use greycat_sys::*;
use std::{ffi, ptr::NonNull, slice};

#[repr(transparent)]
pub struct GcString(NonNull<gc_core_string_t>);

impl GcString {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { gc_core_string__buffer(self.0.as_ptr()) as _ }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { gc_core_string__size(self.0.as_ptr()) as _ }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.as_ptr(), self.len())) }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<&str> for GcString {
    fn from(value: &str) -> Self {
        let ptr = value.as_ptr() as *const ffi::c_char;
        let len = value.len();
        let s = unsafe { gc_core_string__create_from(ptr, len as u64) };
        unsafe { Self::wrap_ptr(s) }
    }
}

impl AsPtr for GcString {
    fn as_ptr(&self) -> *const gc_object_t {
        self.0.as_ptr() as _
    }
}

impl AsPtrMut for GcString {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t {
        self.0.as_ptr() as _
    }
}

impl WrapPtr<gc_object_t> for GcString {
    unsafe fn wrap_ptr<'a>(ptr: *mut gc_object_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr as _) })
    }
}

impl WrapPtr<gc_core_string_t> for GcString {
    unsafe fn wrap_ptr(ptr: *mut gc_core_string_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }
}

impl AsRef<str> for GcString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
