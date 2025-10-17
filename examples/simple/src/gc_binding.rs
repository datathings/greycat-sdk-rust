#![allow(non_snake_case)]

use greycat::prelude::*;

#[global_allocator]
static GLOBAL: greycat::GreyCatAlloc = greycat::GreyCatAlloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gc_lib_simple__link(
    prog: *mut ::greycat::sys::gc_program_t,
    lib: *mut ::greycat::sys::gc_program_library_t,
) -> bool {
    let prog = ::greycat::GcProgramMut(prog);

    unsafe {
        ::greycat::sys::gc_program_library__set_lib_hooks(
            lib,
            Some(simple_start),
            Some(simple_stop),
        )
    };

    let calc = prog.resolve_module("calc");

    prog.link_mod_fn(calc, "sum", simple_calc__sum);
    prog.link_mod_fn(calc, "sub", simple_calc__sub);

    let calc_DoubleInputCalc = prog.resolve_type(calc, "DoubleInputCalc");
    prog.configure_type(
        calc_DoubleInputCalc,
        std::mem::size_of::<crate::calc::DoubleInputCalc>(),
        Some(simple_calc_DoubleInputCalc_finalize),
    );
    prog.link_type_fn(
        calc_DoubleInputCalc,
        "sum",
        simple_calc_DoubleInputCalc__sum,
    );
    prog.link_type_fn(
        calc_DoubleInputCalc,
        "sub",
        simple_calc_DoubleInputCalc__sub,
    );

    true
}

pub(crate) struct SimpleLibrary;

pub(crate) trait LibraryLifecycle<T> {
    fn start(prog: ::greycat::GcProgramMut) -> Result<Option<T>, ()> {
        let _ = prog;
        Ok(None)
    }
    fn stop(prog: ::greycat::GcProgramMut, userdata: Option<T>) -> Result<(), ()> {
        let _ = prog;
        let _ = userdata;
        Ok(())
    }
    fn load(ptr: *mut *mut ::core::ffi::c_void) -> Option<T> {
        let data = unsafe { *ptr } as *mut T;
        if data.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(data) })
        }
    }
}

unsafe extern "C" fn simple_start(
    _lib: *mut ::greycat::sys::gc_program_library_t,
    prog: *mut ::greycat::sys::gc_program_t,
    user_data: *mut *mut ::core::ffi::c_void,
) -> bool {
    let prog = ::greycat::GcProgramMut(prog);
    match SimpleLibrary::start(prog) {
        Ok(Some(data)) => {
            let data = Box::new(data);
            unsafe { *user_data = Box::into_raw(data) as *mut _ };
            true
        }
        Ok(None) => true,
        Err(_) => false,
    }
}

unsafe extern "C" fn simple_stop(
    _lib: *mut ::greycat::sys::gc_program_library_t,
    prog: *mut ::greycat::sys::gc_program_t,
    user_data: *mut *mut ::core::ffi::c_void,
) -> bool {
    let prog = ::greycat::GcProgramMut(prog);
    SimpleLibrary::stop(prog, SimpleLibrary::load(user_data)).is_ok()
}

unsafe extern "C" fn simple_calc_DoubleInputCalc_finalize(
    ptr: *mut ::greycat::sys::gc_object_t,
    ctx: *mut ::greycat::sys::gc_machine_t,
) {
    let this = crate::calc::DoubleInputCalc::from_object_ptr(ptr);
    let ctx = ::greycat::GcMachine(ctx);
    crate::calc::DoubleInputCalc::finalize(this, ctx);
}

unsafe extern "C" fn simple_calc__sum(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let a = unsafe { ctx.get_param(0) };
    let b = unsafe { ctx.get_param(1) };
    ctx.set_result(crate::calc::sum(a, b));
}

unsafe extern "C" fn simple_calc__sub(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let a = unsafe { ctx.get_param(0) };
    let b = unsafe { ctx.get_param(1) };
    ctx.set_result(crate::calc::sub(a, b));
}

unsafe extern "C" fn simple_calc_DoubleInputCalc__sum(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let this = unsafe { ctx.get_self_mut::<crate::calc::DoubleInputCalc>() };
    ctx.set_result(crate::calc::DoubleInputCalc::sum(this, ctx));
}

unsafe extern "C" fn simple_calc_DoubleInputCalc__sub(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let this = unsafe { ctx.get_self_mut::<crate::calc::DoubleInputCalc>() };
    ctx.set_result(crate::calc::DoubleInputCalc::sub(this, ctx));
}

impl crate::calc::DoubleInputCalc {
    pub fn a(&self, ctx: ::greycat::GcMachine) -> i64 {
        unsafe { self.__header.get_at(0, ctx) }
    }

    pub fn b(&self, ctx: ::greycat::GcMachine) -> i64 {
        unsafe { self.__header.get_at(1, ctx) }
    }
}
