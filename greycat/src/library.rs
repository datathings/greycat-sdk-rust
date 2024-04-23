use greycat_sys::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Library(*mut gc_program_library_t);

impl From<*mut gc_program_library_t> for Library {
    #[inline(always)]
    fn from(value: *mut gc_program_library_t) -> Self {
        Self(value)
    }
}

impl Library {
    pub fn configure(&mut self, start: gc_lifecycle_function_t, stop: gc_lifecycle_function_t) {
        unsafe { gc_program_library__configure(self.0, start, stop) }
    }
}
