use greycat::GcLibrary;

mod csv2;
mod gc;

pub(crate) struct Library;

impl GcLibrary for Library {
    fn init(prog: greycat::GcProgramMut) {
        prog.configure_type::<csv2::CsvReader>(gc::csv2_CsvReader(), csv2::CsvReader_finalize);
    }
}
