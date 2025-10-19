mod calc;
mod gc_bindings;
mod io2;

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct LibState;

impl greycat::GcLibrary for LibState {
    fn start(_prog: greycat::GcProgramMut) -> Result<Option<Self>, ()>
    where
        Self: Sized,
    {
        println!("start");
        Ok(None)
    }

    fn stop(_prog: greycat::GcProgramMut, userdata: Option<Self>) -> Result<(), ()>
    where
        Self: Sized,
    {
        println!("stop {userdata:?}");
        Ok(())
    }
}
