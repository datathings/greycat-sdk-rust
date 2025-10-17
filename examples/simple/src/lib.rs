use core::f64;

mod calc;
mod gc_binding;

#[derive(Debug)]
#[allow(unused)]
struct State {
    a: i32,
    b: f64,
}

impl gc_binding::LibraryLifecycle<State> for gc_binding::SimpleLibrary {
    fn start(_: greycat::GcProgramMut) -> Result<Option<State>, ()> {
        println!("start");
        Ok(Some(State {
            a: 42,
            b: f64::consts::PI,
        }))
    }

    fn stop(_: greycat::GcProgramMut, userdata: Option<State>) -> Result<(), ()> {
        println!("stop {userdata:?}");
        Ok(())
    }
}
