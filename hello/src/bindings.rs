#![allow(unused)]
#![allow(non_snake_case)]

use crate::hello;
use greycat2 as gc;
use greycat2::sys::*;

/// # Safety
/// This is called by GreyCat to configure the library
/// We can't guaranty anything besides the fact that we are expecting this specific signature
#[no_mangle]
pub unsafe extern "C" fn gc_lib_hello__link(
    prog: *mut gc_program_t,
    lib: *mut gc_program_library_t,
) -> bool {
    // gc_program_library__set_lib_hooks(lib, None, None);
    let prog = gc::ProgramMut::from(prog);
    let hello = prog.resolve_module("hello");
    let hello_CsvReader = prog.resolve_type(hello, "CsvReader");
    prog.configure_type(
        hello_CsvReader,
        std::mem::size_of::<hello::CsvReader>() as u32,
        hello_hello_CsvReader__finalize,
    );
    prog.link_type_fn(hello_CsvReader, "can_read", hello_hello_CsvReader__can_read);
    prog.link_type_fn(hello_CsvReader, "read", hello_hello_CsvReader__read);
    true
}

unsafe extern "C" fn hello_hello_CsvReader__finalize(
    this: *mut gc_object_t,
    ptr: *mut gc_machine_t,
) {
    let ctx = gc::Machine::from(ptr);
    let this = &mut *(this as *mut hello::CsvReader);
    if this.__initialized {
        hello::CsvReader::__finalize(this)
    }
}

unsafe extern "C" fn hello_hello_CsvReader__can_read(ptr: *mut gc_machine_t) {
    let ctx = gc::Machine::from(ptr);
    let this = &mut *ctx.get_self::<hello::CsvReader>();
    if !this.__initialized {
        hello::CsvReader::__initialize(this, ctx);
    }
    match hello::CsvReader::can_read(this, ctx) {
        Ok(value) => ctx.set_result(value),
        Err(message) => ctx.set_error(&message),
    }
}

unsafe extern "C" fn hello_hello_CsvReader__read(ptr: *mut gc_machine_t) {
    let ctx = gc::Machine::from(ptr);
    let this = &mut *ctx.get_self::<hello::CsvReader>();
    if !this.__initialized {
        hello::CsvReader::__initialize(this, ctx);
    }
    match hello::CsvReader::read(this, ctx) {
        Ok(value) => ctx.set_result(value),
        Err(message) => ctx.set_error(&message),
    }
}

impl hello::CsvReader {
    pub fn path(&self, ctx: gc::Machine) -> gc::String {
        let mut slot_type = gc_type_null;
        let slot = unsafe { gc_object__get_at(&self.header, 0, &mut slot_type, ctx.0) };
        let path = unsafe { slot.__1.object as *mut gc_core_string_t };
        gc::String::from(path)
    }
}
