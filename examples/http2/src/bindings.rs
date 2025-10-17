#![allow(unused)]
#![allow(non_snake_case)]

use crate::bindings;
use greycat::sys::*;

use greycat as gc;

#[no_mangle]
pub unsafe extern "C" fn gc_lib_http2__link(
    prog: *mut gc_program_t,
    lib: *mut gc_program_library_t,
) -> bool {
    let prog = gc::GcProgramMut(prog);
    let _http2 = prog.resolve_module("http2");
    true
}
