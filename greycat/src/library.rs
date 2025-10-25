use crate::GcProgramMut;

pub trait GcLibrary<E = ()> {
    /// Called during the link phase of the library
    ///
    /// This is a good place to configure native type if needed.
    fn init(prog: GcProgramMut) {
        let _ = prog;
    }

    /// Called when the library starts.
    ///
    /// The optional returned value will be store by GreyCat and given back at the `stop` phase.
    fn start(prog: GcProgramMut) -> Result<Option<Self>, E>
    where
        Self: Sized,
    {
        let _ = prog;
        Ok(None)
    }

    /// Called when the library stops.
    ///
    /// The optional `userdata` is the returned value of the `start` call.
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
    ///
    /// **Your are not supposed to override this function unless you know what you are doing**
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
