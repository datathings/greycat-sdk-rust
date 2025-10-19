use std::{ptr::NonNull, slice};

use greycat_sys::*;

use crate::{
    object::{AsGcObject, GcObjectRef},
    FromPtr,
};

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

impl AsGcObject for GcString {
    #[inline(always)]
    fn as_object(&self) -> GcObjectRef {
        GcObjectRef(self.0.as_ptr() as _)
    }
}

impl FromPtr<gc_object_t> for GcString {
    unsafe fn from_ptr<'a>(ptr: *mut gc_object_t) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr as _) })
    }
}

impl FromPtr<gc_core_string_t> for GcString {
    unsafe fn from_ptr(ptr: *mut gc_core_string_t) -> Self {
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
