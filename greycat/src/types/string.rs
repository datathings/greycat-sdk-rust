use std::{ptr::NonNull, slice};

use greycat_sys::*;

use crate::{
    ffi::FromPtr,
    object::{AsGcObject, FromObjectPtr, GcObjectRef},
};

pub struct GcString(NonNull<gc_core_string_t>);

impl std::fmt::Display for GcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromPtr for GcString {
    type CType = gc_core_string_t;

    fn from_ptr(ptr: *mut Self::CType) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }
}

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

impl FromObjectPtr for GcString {
    fn from_object_ptr<'a>(ptr: *mut gc_object_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }
}

impl AsRef<str> for GcString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
