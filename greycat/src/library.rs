use crate::GcProgramMut;

pub trait GcLibrary<E = ()> {
    fn start(prog: GcProgramMut) -> Result<Option<Self>, E>
    where
        Self: Sized,
    {
        let _ = prog;
        Ok(None)
    }
    fn stop(prog: GcProgramMut, userdata: Option<Self>) -> Result<(), E>
    where
        Self: Sized,
    {
        let _ = prog;
        let _ = userdata;
        Ok(())
    }
    /// # Safety
    /// Always give the library userdata pointer to this function
    unsafe fn load(ptr: *mut *mut ::core::ffi::c_void) -> Option<Self>
    where
        Self: Sized,
    {
        let data = unsafe { *ptr } as *mut Self;
        if data.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(data) })
        }
    }
}
