#![allow(non_snake_case)]

use greycat::prelude::*;

#[global_allocator]
static GLOBAL: ::greycat::GreyCatAlloc = ::greycat::GreyCatAlloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gc_lib_csv2__link(
    prog: *mut ::greycat::sys::gc_program_t,
    _lib: *mut ::greycat::sys::gc_program_library_t,
) -> bool {
    let prog = ::greycat::GcProgramMut(prog);
    let csv2 = prog.resolve_module("csv2");

    let csv2_CsvReader = prog.resolve_type(csv2, "CsvReader");
    prog.configure_type(
        csv2_CsvReader,
        std::mem::size_of::<crate::csv2::CsvReader>(),
        Some(csv2_csv2_CsvReader_finalize),
    );
    prog.link_type_fn(csv2_CsvReader, "can_read", csv2_csv2_CsvReader__can_read);
    prog.link_type_fn(csv2_CsvReader, "read", csv2_csv2_CsvReader__read);
    true
}

unsafe extern "C" fn csv2_csv2_CsvReader_finalize(
    this: *mut ::greycat::sys::gc_object_t,
    ctx: *mut ::greycat::sys::gc_machine_t,
) {
    let this = crate::csv2::CsvReader::from_object_ptr(this);
    let ctx = ::greycat::GcMachine(ctx);
    crate::csv2::CsvReader::finalize(this, ctx);
}

unsafe extern "C" fn csv2_csv2_CsvReader__can_read(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let this = unsafe { ctx.get_self_mut() };
    match crate::csv2::CsvReader::can_read(this, ctx) {
        Ok(value) => ctx.set_result(value),
        Err(message) => ctx.set_error(message.to_string()),
    }
}

unsafe extern "C" fn csv2_csv2_CsvReader__read(ctx: *mut ::greycat::sys::gc_machine_t) {
    let ctx = ::greycat::GcMachine(ctx);
    let this = unsafe { ctx.get_self_mut() };
    match crate::csv2::CsvReader::read(this, ctx) {
        Ok(value) => ctx.set_result(value),
        Err(message) => ctx.set_error(message.to_string()),
    }
}

impl crate::csv2::CsvReader {
    #[allow(clippy::mut_from_ref)]
    pub fn path(&self, ctx: ::greycat::GcMachine) -> &mut ::greycat::GcString {
        unsafe { self.__header.get_at(0, ctx) }
    }
}
