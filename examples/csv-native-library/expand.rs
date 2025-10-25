#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use greycat::GcLibrary;
mod csv2 {
    use std::fs::File;
    use greycat::prelude::*;
    use crate::gc;
    #[repr(C)]
    pub struct CsvReader {
        pub(crate) __header: ::greycat::GcObjectOwned,
        records: Option<Box<csv::StringRecordsIntoIter<File>>>,
        record: Option<Result<csv::StringRecord, csv::Error>>,
    }
    impl ::greycat::object::AsPtr for CsvReader {
        fn as_ptr(&self) -> *const ::greycat::sys::gc_object_t {
            self as *const Self as *const _
        }
    }
    impl ::greycat::object::AsPtrMut for CsvReader {
        fn as_ptr_mut(&mut self) -> *mut ::greycat::sys::gc_object_t {
            self as *mut Self as *mut _
        }
    }
    #[allow(non_snake_case)]
    unsafe extern "C" fn __gc_CsvReader_finalize_fn(
        this: *mut ::greycat::sys::gc_object_t,
        ctx: *mut ::greycat::sys::gc_machine_t,
    ) {
        let ctx = ::greycat::GcMachine(ctx);
        let this = unsafe { &mut *(this as *mut CsvReader) };
        CsvReader::finalize(this, ctx);
    }
    #[allow(non_upper_case_globals)]
    pub(crate) static _gc_CsvReader_finalize: Option<::greycat::GcObjectFinalizeFn> = Some(
        __gc_CsvReader_finalize_fn,
    );
    impl Object for CsvReader {
        fn finalize(&mut self, _ctx: GcMachine) {
            self.records.take();
        }
    }
    impl CsvReader {
        fn initialize_reader(&mut self, ctx: GcMachine) -> GcResult<()> {
            let path: GcString = unsafe { self.get_at(gc::csv2_CsvReader_path, ctx) };
            let options: Option<GcObject> = unsafe {
                self.get_at(gc::csv2_CsvReader_options, ctx)
            };
            let file = File::open(path.as_str())?;
            let mut builder = match options {
                Some(opts) => opts.reader_builder(ctx),
                None => csv::ReaderBuilder::new(),
            };
            let reader = builder.buffer_capacity(8192).from_reader(file);
            self.records.replace(Box::new(reader.into_records()));
            Ok(())
        }
    }
    impl CsvReader {
        pub fn can_read(&mut self, ctx: GcMachine) -> GcResult<bool> {
            let records = match self.records.as_ref() {
                Some(records) => records,
                None => {
                    self.initialize_reader(ctx)?;
                    self.records.as_ref().unwrap()
                }
            };
            let is_done = records.reader().is_done();
            {
                ::std::io::_print(format_args!("> is_done={0}\n", is_done));
            };
            Ok(!is_done)
        }
        pub fn read(&mut self, ctx: GcMachine) -> GcResult<Option<GcObject>> {
            let records = match self.records.as_mut() {
                Some(records) => records,
                None => {
                    self.initialize_reader(ctx)?;
                    self.records.as_mut().unwrap()
                }
            };
            let record = records.next();
            {
                ::std::io::_print(format_args!("> record={0:?}\n", record));
            };
            match record {
                Some(record) => {
                    let record = record?;
                    let mut row = GcArray::new(ctx);
                    for field in record.iter() {
                        if let Ok(value) = field.parse::<i64>() {
                            row.add(value, ctx);
                        } else if let Ok(value) = field.parse::<f64>() {
                            row.add(value, ctx);
                        } else if let Ok(value) = field.parse::<bool>() {
                            row.add(value, ctx);
                        } else {
                            row.add(field, ctx);
                        }
                    }
                    Ok(Some(row.as_object()))
                }
                None => Ok(None),
            }
        }
    }
    #[allow(non_snake_case)]
    pub(crate) unsafe extern "C" fn _gc_CsvReader__can_read(
        ctx: *mut ::greycat::sys::gc_machine_t,
    ) {
        let ctx = ::greycat::GcMachine(ctx);
        let this = unsafe { ctx.get_self_mut::<CsvReader>() };
        match CsvReader::can_read(this, ctx) {
            Ok(value) => ctx.set_result(value),
            Err(err) => ctx.set_error(err),
        }
    }
    #[allow(non_snake_case)]
    pub(crate) unsafe extern "C" fn _gc_CsvReader__read(
        ctx: *mut ::greycat::sys::gc_machine_t,
    ) {
        let ctx = ::greycat::GcMachine(ctx);
        let this = unsafe { ctx.get_self_mut::<CsvReader>() };
        match CsvReader::read(this, ctx) {
            Ok(value) => ctx.set_result(value),
            Err(err) => ctx.set_error(err),
        }
    }
    trait CsvOptions {
        fn reader_builder(&self, ctx: GcMachine) -> csv::ReaderBuilder;
        fn has_header(&self, ctx: GcMachine) -> bool;
        fn delimiter(&self, ctx: GcMachine) -> u8;
        fn flexible(&self, ctx: GcMachine) -> bool;
        fn trim(&self, ctx: GcMachine) -> Option<csv::Trim>;
    }
    impl CsvOptions for GcObject {
        fn reader_builder(&self, ctx: GcMachine) -> csv::ReaderBuilder {
            let mut builder = csv::ReaderBuilder::new();
            builder
                .has_headers(self.has_header(ctx))
                .delimiter(self.delimiter(ctx))
                .flexible(self.flexible(ctx));
            if let Some(trim) = self.trim(ctx) {
                builder.trim(trim);
            }
            builder
        }
        fn has_header(&self, ctx: GcMachine) -> bool {
            let value: Option<bool> = unsafe {
                self.get_at(gc::csv2_CsvOptions_has_header, ctx)
            };
            value.unwrap_or(true)
        }
        fn delimiter(&self, ctx: GcMachine) -> u8 {
            let value: Option<char> = unsafe {
                self.get_at(gc::csv2_CsvOptions_delimiter, ctx)
            };
            match value {
                Some(value) => value as u8,
                None => b',',
            }
        }
        fn flexible(&self, ctx: GcMachine) -> bool {
            let value: Option<bool> = unsafe {
                self.get_at(gc::csv2_CsvOptions_flexible, ctx)
            };
            value.unwrap_or_default()
        }
        fn trim(&self, ctx: GcMachine) -> Option<csv::Trim> {
            let value: Option<GcEnum> = unsafe {
                self.get_at(gc::csv2_CsvOptions_trim, ctx)
            };
            match value {
                Some(value) => {
                    match value.0.1 {
                        gc::csv2_CsvTrim_All => Some(csv::Trim::All),
                        gc::csv2_CsvTrim_Fields => Some(csv::Trim::Fields),
                        gc::csv2_CsvTrim_Headers => Some(csv::Trim::Headers),
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                None => None,
            }
        }
    }
}
mod gc {
    #![allow(non_snake_case, non_upper_case_globals, unused)]
    static GLOBAL: ::greycat::GreyCatAlloc = ::greycat::GreyCatAlloc;
    const _: () = {
        #[rustc_std_internal_symbol]
        unsafe fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc(
                &GLOBAL,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) -> () {
            ::core::alloc::GlobalAlloc::dealloc(
                &GLOBAL,
                ptr,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_realloc(
            ptr: *mut u8,
            size: usize,
            align: usize,
            new_size: usize,
        ) -> *mut u8 {
            ::core::alloc::GlobalAlloc::realloc(
                &GLOBAL,
                ptr,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
                new_size,
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc_zeroed(
                &GLOBAL,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
    };
    static mut _core_GeoCircle: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_GeoCircle() -> ::greycat::GcTypeId {
        unsafe { _core_GeoCircle }
    }
    static mut _core_Tuple__int_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple__int_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__int_any_null }
    }
    static mut _core_VectorIndex: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_VectorIndex() -> ::greycat::GcTypeId {
        unsafe { _core_VectorIndex }
    }
    static mut _core_Array__Role: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Role() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Role }
    }
    static mut _core_str: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_str() -> ::greycat::GcTypeId {
        unsafe { _core_str }
    }
    static mut _core_Array__Permission: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Permission() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Permission }
    }
    static mut _core_Array__Frame: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Frame() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Frame }
    }
    static mut _core_Map__String_MediaTypeObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_MediaTypeObject() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_MediaTypeObject }
    }
    static mut _core_Array__ErrorFrame: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__ErrorFrame() -> ::greycat::GcTypeId {
        unsafe { _core_Array__ErrorFrame }
    }
    static mut _core_float: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_float() -> ::greycat::GcTypeId {
        unsafe { _core_float }
    }
    static mut _core_t3f: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t3f() -> ::greycat::GcTypeId {
        unsafe { _core_t3f }
    }
    static mut _core_GeoBox: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_GeoBox() -> ::greycat::GcTypeId {
        unsafe { _core_GeoBox }
    }
    static mut _core_Array__ResponseObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__ResponseObject() -> ::greycat::GcTypeId {
        unsafe { _core_Array__ResponseObject }
    }
    static mut _core_Array__PathItemObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__PathItemObject() -> ::greycat::GcTypeId {
        unsafe { _core_Array__PathItemObject }
    }
    static mut _core_nodeTime: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeTime() -> ::greycat::GcTypeId {
        unsafe { _core_nodeTime }
    }
    static mut _core_Table__GaussianProfileSlot_null: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Table__GaussianProfileSlot_null() -> ::greycat::GcTypeId {
        unsafe { _core_Table__GaussianProfileSlot_null }
    }
    static mut _core_nodeIndexBucket: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeIndexBucket() -> ::greycat::GcTypeId {
        unsafe { _core_nodeIndexBucket }
    }
    static mut _core_Array__node_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__node_null() -> ::greycat::GcTypeId {
        unsafe { _core_Array__node_null }
    }
    static mut _core_Array__HistogramBin: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__HistogramBin() -> ::greycat::GcTypeId {
        unsafe { _core_Array__HistogramBin }
    }
    static mut _core_String: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_String() -> ::greycat::GcTypeId {
        unsafe { _core_String }
    }
    static mut _core_NodeInfo__int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_NodeInfo__int() -> ::greycat::GcTypeId {
        unsafe { _core_NodeInfo__int }
    }
    static mut _core_field: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_field() -> ::greycat::GcTypeId {
        unsafe { _core_field }
    }
    static mut _core_nodeList: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeList() -> ::greycat::GcTypeId {
        unsafe { _core_nodeList }
    }
    static mut _core_t3: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t3() -> ::greycat::GcTypeId {
        unsafe { _core_t3 }
    }
    static mut _core_time: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_time() -> ::greycat::GcTypeId {
        unsafe { _core_time }
    }
    static mut _core_Table__Tuple__float_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Table__Tuple__float_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Table__Tuple__float_any_null }
    }
    static mut _core_Array__MediaTypeObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Array__MediaTypeObject() -> ::greycat::GcTypeId {
        unsafe { _core_Array__MediaTypeObject }
    }
    static mut _core_Array__SearchResult: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__SearchResult() -> ::greycat::GcTypeId {
        unsafe { _core_Array__SearchResult }
    }
    static mut _core_Tensor: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tensor() -> ::greycat::GcTypeId {
        unsafe { _core_Tensor }
    }
    static mut _core_Array__HeaderObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__HeaderObject() -> ::greycat::GcTypeId {
        unsafe { _core_Array__HeaderObject }
    }
    static mut _core_geo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_geo() -> ::greycat::GcTypeId {
        unsafe { _core_geo }
    }
    static mut _core_Map__String_UserCredential: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_UserCredential() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_UserCredential }
    }
    static mut _core_NodeInfo__time: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_NodeInfo__time() -> ::greycat::GcTypeId {
        unsafe { _core_NodeInfo__time }
    }
    static mut _core_Buffer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Buffer() -> ::greycat::GcTypeId {
        unsafe { _core_Buffer }
    }
    static mut _core_Date: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Date() -> ::greycat::GcTypeId {
        unsafe { _core_Date }
    }
    static mut _core_Array__UserGroupPolicy: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Array__UserGroupPolicy() -> ::greycat::GcTypeId {
        unsafe { _core_Array__UserGroupPolicy }
    }
    static mut _core_ErrorFrame: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_ErrorFrame() -> ::greycat::GcTypeId {
        unsafe { _core_ErrorFrame }
    }
    static mut _core_TableColumnMapping: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_TableColumnMapping() -> ::greycat::GcTypeId {
        unsafe { _core_TableColumnMapping }
    }
    static mut _core_Map__String_PathItemObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_PathItemObject() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_PathItemObject }
    }
    static mut _core_Array__PeriodicTask: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__PeriodicTask() -> ::greycat::GcTypeId {
        unsafe { _core_Array__PeriodicTask }
    }
    static mut _core_Tuple__time_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple__time_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__time_any_null }
    }
    static mut _core_Array__DateTuple: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__DateTuple() -> ::greycat::GcTypeId {
        unsafe { _core_Array__DateTuple }
    }
    static mut _core_Array__SchemaObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__SchemaObject() -> ::greycat::GcTypeId {
        unsafe { _core_Array__SchemaObject }
    }
    static mut _core_Array__File: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__File() -> ::greycat::GcTypeId {
        unsafe { _core_Array__File }
    }
    static mut _core_GeoPoly: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_GeoPoly() -> ::greycat::GcTypeId {
        unsafe { _core_GeoPoly }
    }
    static mut _core_nodeList__nodeList__VectorLeaf: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_nodeList__nodeList__VectorLeaf() -> ::greycat::GcTypeId {
        unsafe { _core_nodeList__nodeList__VectorLeaf }
    }
    static mut _core_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_null() -> ::greycat::GcTypeId {
        unsafe { _core_null }
    }
    static mut _core_type: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_type() -> ::greycat::GcTypeId {
        unsafe { _core_type }
    }
    static mut _core_t4f: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t4f() -> ::greycat::GcTypeId {
        unsafe { _core_t4f }
    }
    static mut _core_Array__NodeInfo__geo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__NodeInfo__geo() -> ::greycat::GcTypeId {
        unsafe { _core_Array__NodeInfo__geo }
    }
    static mut _core_node: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_node() -> ::greycat::GcTypeId {
        unsafe { _core_node }
    }
    static mut _core_Error: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Error() -> ::greycat::GcTypeId {
        unsafe { _core_Error }
    }
    static mut _core_Array__NodeInfo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__NodeInfo() -> ::greycat::GcTypeId {
        unsafe { _core_Array__NodeInfo }
    }
    static mut _core_Array__TableColumnMapping: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Array__TableColumnMapping() -> ::greycat::GcTypeId {
        unsafe { _core_Array__TableColumnMapping }
    }
    static mut _core_Array__Job: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Job() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Job }
    }
    static mut _core_Vector: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Vector() -> ::greycat::GcTypeId {
        unsafe { _core_Vector }
    }
    static mut _core_Array__geo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__geo() -> ::greycat::GcTypeId {
        unsafe { _core_Array__geo }
    }
    static mut _core_Array__String: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__String() -> ::greycat::GcTypeId {
        unsafe { _core_Array__String }
    }
    static mut _core_Array__any: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__any() -> ::greycat::GcTypeId {
        unsafe { _core_Array__any }
    }
    static mut _core_nodeIndex: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeIndex() -> ::greycat::GcTypeId {
        unsafe { _core_nodeIndex }
    }
    static mut _core_Tuple__int_VectorLeaf: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple__int_VectorLeaf() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__int_VectorLeaf }
    }
    static mut _core_Tuple__float_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple__float_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__float_any_null }
    }
    static mut _core_Map__String_HeaderObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_HeaderObject() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_HeaderObject }
    }
    static mut _core_NodeInfo__geo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_NodeInfo__geo() -> ::greycat::GcTypeId {
        unsafe { _core_NodeInfo__geo }
    }
    static mut _core_Map__String_SchemaObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_SchemaObject() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_SchemaObject }
    }
    static mut _core_Map__String_int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Map__String_int() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_int }
    }
    static mut _core_Array__Quantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Quantizer() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Quantizer }
    }
    static mut _core_Map__String_ResponseObject: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Map__String_ResponseObject() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_ResponseObject }
    }
    static mut _core_Array__Variable: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Variable() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Variable }
    }
    static mut _core_Tuple__int_nodeList__VectorLeaf: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Tuple__int_nodeList__VectorLeaf() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__int_nodeList__VectorLeaf }
    }
    static mut _core_Array__CsvColumnStatistics: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Array__CsvColumnStatistics() -> ::greycat::GcTypeId {
        unsafe { _core_Array__CsvColumnStatistics }
    }
    static mut _core_Table: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Table() -> ::greycat::GcTypeId {
        unsafe { _core_Table }
    }
    static mut _core_MathConstants: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_MathConstants() -> ::greycat::GcTypeId {
        unsafe { _core_MathConstants }
    }
    static mut _core_Array__Task: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__Task() -> ::greycat::GcTypeId {
        unsafe { _core_Array__Task }
    }
    static mut _core_bool: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_bool() -> ::greycat::GcTypeId {
        unsafe { _core_bool }
    }
    static mut _core_Array__nodeTime: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__nodeTime() -> ::greycat::GcTypeId {
        unsafe { _core_Array__nodeTime }
    }
    static mut _core_Array__nodeList: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__nodeList() -> ::greycat::GcTypeId {
        unsafe { _core_Array__nodeList }
    }
    static mut _core_Array__nodeIndex: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__nodeIndex() -> ::greycat::GcTypeId {
        unsafe { _core_Array__nodeIndex }
    }
    static mut _core_Map__any_int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Map__any_int() -> ::greycat::GcTypeId {
        unsafe { _core_Map__any_int }
    }
    static mut _core_Array__SecurityEntity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__SecurityEntity() -> ::greycat::GcTypeId {
        unsafe { _core_Array__SecurityEntity }
    }
    static mut _core_Array__NodeInfo__int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__NodeInfo__int() -> ::greycat::GcTypeId {
        unsafe { _core_Array__NodeInfo__int }
    }
    static mut _core_Array__int_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__int_null() -> ::greycat::GcTypeId {
        unsafe { _core_Array__int_null }
    }
    static mut _core_Array__int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__int() -> ::greycat::GcTypeId {
        unsafe { _core_Array__int }
    }
    static mut _core_duration: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_duration() -> ::greycat::GcTypeId {
        unsafe { _core_duration }
    }
    static mut _core_nodeList__VectorLeaf: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeList__VectorLeaf() -> ::greycat::GcTypeId {
        unsafe { _core_nodeList__VectorLeaf }
    }
    static mut _core_Array: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array() -> ::greycat::GcTypeId {
        unsafe { _core_Array }
    }
    static mut _core_Array__DayOfWeek: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__DayOfWeek() -> ::greycat::GcTypeId {
        unsafe { _core_Array__DayOfWeek }
    }
    static mut _core_Tuple: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple }
    }
    static mut _core_Map: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Map() -> ::greycat::GcTypeId {
        unsafe { _core_Map }
    }
    static mut _core_nodeGeo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeGeo() -> ::greycat::GcTypeId {
        unsafe { _core_nodeGeo }
    }
    static mut _core_Array__field: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__field() -> ::greycat::GcTypeId {
        unsafe { _core_Array__field }
    }
    static mut _core_any: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_any() -> ::greycat::GcTypeId {
        unsafe { _core_any }
    }
    static mut _core_char: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_char() -> ::greycat::GcTypeId {
        unsafe { _core_char }
    }
    static mut _core_Array__nodeGeo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__nodeGeo() -> ::greycat::GcTypeId {
        unsafe { _core_Array__nodeGeo }
    }
    static mut _core_nodeTimeCursor: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_nodeTimeCursor() -> ::greycat::GcTypeId {
        unsafe { _core_nodeTimeCursor }
    }
    static mut _core_Array__UserCredential: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__UserCredential() -> ::greycat::GcTypeId {
        unsafe { _core_Array__UserCredential }
    }
    static mut _core_int: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_int() -> ::greycat::GcTypeId {
        unsafe { _core_int }
    }
    static mut _core_Map__String_String: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Map__String_String() -> ::greycat::GcTypeId {
        unsafe { _core_Map__String_String }
    }
    static mut _core_Table__Tuple__time_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn core_Table__Tuple__time_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Table__Tuple__time_any_null }
    }
    static mut _core_t2: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t2() -> ::greycat::GcTypeId {
        unsafe { _core_t2 }
    }
    static mut _core_t2f: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t2f() -> ::greycat::GcTypeId {
        unsafe { _core_t2f }
    }
    static mut _core_SearchResult: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_SearchResult() -> ::greycat::GcTypeId {
        unsafe { _core_SearchResult }
    }
    static mut _core_function: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_function() -> ::greycat::GcTypeId {
        unsafe { _core_function }
    }
    static mut _core_Array__float: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__float() -> ::greycat::GcTypeId {
        unsafe { _core_Array__float }
    }
    static mut _core_VectorLeaf: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_VectorLeaf() -> ::greycat::GcTypeId {
        unsafe { _core_VectorLeaf }
    }
    static mut _core_NodeInfo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_NodeInfo() -> ::greycat::GcTypeId {
        unsafe { _core_NodeInfo }
    }
    static mut _core_Tuple__geo_any_null: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Tuple__geo_any_null() -> ::greycat::GcTypeId {
        unsafe { _core_Tuple__geo_any_null }
    }
    static mut _core_t4: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_t4() -> ::greycat::GcTypeId {
        unsafe { _core_t4 }
    }
    static mut _core_Array__NodeInfo__time: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn core_Array__NodeInfo__time() -> ::greycat::GcTypeId {
        unsafe { _core_Array__NodeInfo__time }
    }
    static mut _runtime_Runtime: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Runtime() -> ::greycat::GcTypeId {
        unsafe { _runtime_Runtime }
    }
    static mut _runtime_ChildProcess: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_ChildProcess() -> ::greycat::GcTypeId {
        unsafe { _runtime_ChildProcess }
    }
    static mut _runtime_Periodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Periodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_Periodicity }
    }
    static mut _runtime_UserGroupPolicy: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_UserGroupPolicy() -> ::greycat::GcTypeId {
        unsafe { _runtime_UserGroupPolicy }
    }
    static mut _runtime_Variable: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Variable() -> ::greycat::GcTypeId {
        unsafe { _runtime_Variable }
    }
    static mut _runtime_OpenApiV3: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_OpenApiV3() -> ::greycat::GcTypeId {
        unsafe { _runtime_OpenApiV3 }
    }
    static mut _runtime_OperationObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_OperationObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_OperationObject }
    }
    static mut _runtime_License: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_License() -> ::greycat::GcTypeId {
        unsafe { _runtime_License }
    }
    static mut _runtime_FixedPeriodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_FixedPeriodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_FixedPeriodicity }
    }
    static mut _runtime_MonthlyPeriodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_MonthlyPeriodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_MonthlyPeriodicity }
    }
    static mut _runtime_RuntimeInfo: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_RuntimeInfo() -> ::greycat::GcTypeId {
        unsafe { _runtime_RuntimeInfo }
    }
    static mut _runtime_LogDataUsage: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_LogDataUsage() -> ::greycat::GcTypeId {
        unsafe { _runtime_LogDataUsage }
    }
    static mut _runtime_Debug: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Debug() -> ::greycat::GcTypeId {
        unsafe { _runtime_Debug }
    }
    static mut _runtime_Scheduler: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Scheduler() -> ::greycat::GcTypeId {
        unsafe { _runtime_Scheduler }
    }
    static mut _runtime_SecurityFields: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_SecurityFields() -> ::greycat::GcTypeId {
        unsafe { _runtime_SecurityFields }
    }
    static mut _runtime_MediaTypeObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_MediaTypeObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_MediaTypeObject }
    }
    static mut _runtime_Log: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Log() -> ::greycat::GcTypeId {
        unsafe { _runtime_Log }
    }
    static mut _runtime_InfoObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_InfoObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_InfoObject }
    }
    static mut _runtime_SecurityPolicy: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_SecurityPolicy() -> ::greycat::GcTypeId {
        unsafe { _runtime_SecurityPolicy }
    }
    static mut _runtime_Task: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Task() -> ::greycat::GcTypeId {
        unsafe { _runtime_Task }
    }
    static mut _runtime_UserGroup: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_UserGroup() -> ::greycat::GcTypeId {
        unsafe { _runtime_UserGroup }
    }
    static mut _runtime_SchemaObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_SchemaObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_SchemaObject }
    }
    static mut _runtime_YearlyPeriodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_YearlyPeriodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_YearlyPeriodicity }
    }
    static mut _runtime_System: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_System() -> ::greycat::GcTypeId {
        unsafe { _runtime_System }
    }
    static mut _runtime_DateTuple: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_DateTuple() -> ::greycat::GcTypeId {
        unsafe { _runtime_DateTuple }
    }
    static mut _runtime_Role: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Role() -> ::greycat::GcTypeId {
        unsafe { _runtime_Role }
    }
    static mut _runtime_ResponseObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_ResponseObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_ResponseObject }
    }
    static mut _runtime_User: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_User() -> ::greycat::GcTypeId {
        unsafe { _runtime_User }
    }
    static mut _runtime_Job: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Job() -> ::greycat::GcTypeId {
        unsafe { _runtime_Job }
    }
    static mut _runtime_Permission: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Permission() -> ::greycat::GcTypeId {
        unsafe { _runtime_Permission }
    }
    static mut _runtime_WeeklyPeriodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_WeeklyPeriodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_WeeklyPeriodicity }
    }
    static mut _runtime_ComponentsObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_ComponentsObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_ComponentsObject }
    }
    static mut _runtime_PeriodicTask: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_PeriodicTask() -> ::greycat::GcTypeId {
        unsafe { _runtime_PeriodicTask }
    }
    static mut _runtime_PeriodicOptions: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_PeriodicOptions() -> ::greycat::GcTypeId {
        unsafe { _runtime_PeriodicOptions }
    }
    static mut _runtime_UserCredential: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_UserCredential() -> ::greycat::GcTypeId {
        unsafe { _runtime_UserCredential }
    }
    static mut _runtime_ChildProcessResult: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_ChildProcessResult() -> ::greycat::GcTypeId {
        unsafe { _runtime_ChildProcessResult }
    }
    static mut _runtime_DailyPeriodicity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_DailyPeriodicity() -> ::greycat::GcTypeId {
        unsafe { _runtime_DailyPeriodicity }
    }
    static mut _runtime_Frame: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_Frame() -> ::greycat::GcTypeId {
        unsafe { _runtime_Frame }
    }
    static mut _runtime_RequestBodyObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_RequestBodyObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_RequestBodyObject }
    }
    static mut _runtime_HeaderObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_HeaderObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_HeaderObject }
    }
    static mut _runtime_OpenApi: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_OpenApi() -> ::greycat::GcTypeId {
        unsafe { _runtime_OpenApi }
    }
    static mut _runtime_PathItemObject: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_PathItemObject() -> ::greycat::GcTypeId {
        unsafe { _runtime_PathItemObject }
    }
    static mut _runtime_OpenIDConnect: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_OpenIDConnect() -> ::greycat::GcTypeId {
        unsafe { _runtime_OpenIDConnect }
    }
    static mut _runtime_SecurityEntity: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn runtime_SecurityEntity() -> ::greycat::GcTypeId {
        unsafe { _runtime_SecurityEntity }
    }
    static mut _io_GcbWriter: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_GcbWriter() -> ::greycat::GcTypeId {
        unsafe { _io_GcbWriter }
    }
    static mut _io_HttpResponse: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_HttpResponse() -> ::greycat::GcTypeId {
        unsafe { _io_HttpResponse }
    }
    static mut _io_Writer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Writer() -> ::greycat::GcTypeId {
        unsafe { _io_Writer }
    }
    static mut _io_GcbReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_GcbReader() -> ::greycat::GcTypeId {
        unsafe { _io_GcbReader }
    }
    static mut _io_CsvStatistics: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvStatistics() -> ::greycat::GcTypeId {
        unsafe { _io_CsvStatistics }
    }
    static mut _io_Http: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Http() -> ::greycat::GcTypeId {
        unsafe { _io_Http }
    }
    static mut _io_Url: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Url() -> ::greycat::GcTypeId {
        unsafe { _io_Url }
    }
    static mut _io_TextReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_TextReader() -> ::greycat::GcTypeId {
        unsafe { _io_TextReader }
    }
    static mut _io_CsvFormat: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvFormat() -> ::greycat::GcTypeId {
        unsafe { _io_CsvFormat }
    }
    static mut _io_JsonWriter: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_JsonWriter() -> ::greycat::GcTypeId {
        unsafe { _io_JsonWriter }
    }
    static mut _io_CsvAnalysisConfig: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvAnalysisConfig() -> ::greycat::GcTypeId {
        unsafe { _io_CsvAnalysisConfig }
    }
    static mut _io_HttpRequest: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_HttpRequest() -> ::greycat::GcTypeId {
        unsafe { _io_HttpRequest }
    }
    static mut _io_FileWalker: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_FileWalker() -> ::greycat::GcTypeId {
        unsafe { _io_FileWalker }
    }
    static mut _io_Csv: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Csv() -> ::greycat::GcTypeId {
        unsafe { _io_Csv }
    }
    static mut _io_Json: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Json() -> ::greycat::GcTypeId {
        unsafe { _io_Json }
    }
    static mut _io_Reader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Reader() -> ::greycat::GcTypeId {
        unsafe { _io_Reader }
    }
    static mut _io_JsonReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_JsonReader() -> ::greycat::GcTypeId {
        unsafe { _io_JsonReader }
    }
    static mut _io_CsvColumnStatistics: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvColumnStatistics() -> ::greycat::GcTypeId {
        unsafe { _io_CsvColumnStatistics }
    }
    static mut _io_File: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_File() -> ::greycat::GcTypeId {
        unsafe { _io_File }
    }
    static mut _io_Smtp: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Smtp() -> ::greycat::GcTypeId {
        unsafe { _io_Smtp }
    }
    static mut _io_Reader__String: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Reader__String() -> ::greycat::GcTypeId {
        unsafe { _io_Reader__String }
    }
    static mut _io_CsvWriter: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvWriter() -> ::greycat::GcTypeId {
        unsafe { _io_CsvWriter }
    }
    static mut _io_CsvSharding: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvSharding() -> ::greycat::GcTypeId {
        unsafe { _io_CsvSharding }
    }
    static mut _io_TextWriter: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_TextWriter() -> ::greycat::GcTypeId {
        unsafe { _io_TextWriter }
    }
    static mut _io_Email: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_Email() -> ::greycat::GcTypeId {
        unsafe { _io_Email }
    }
    static mut _io_XmlReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_XmlReader() -> ::greycat::GcTypeId {
        unsafe { _io_XmlReader }
    }
    static mut _io_CsvReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn io_CsvReader() -> ::greycat::GcTypeId {
        unsafe { _io_CsvReader }
    }
    static mut _util_ProgressTracker: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_ProgressTracker() -> ::greycat::GcTypeId {
        unsafe { _util_ProgressTracker }
    }
    static mut _util_Random: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Random() -> ::greycat::GcTypeId {
        unsafe { _util_Random }
    }
    static mut _util_SlidingWindow: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_SlidingWindow() -> ::greycat::GcTypeId {
        unsafe { _util_SlidingWindow }
    }
    static mut _util_LinearQuantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_LinearQuantizer() -> ::greycat::GcTypeId {
        unsafe { _util_LinearQuantizer }
    }
    static mut _util_Stack: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Stack() -> ::greycat::GcTypeId {
        unsafe { _util_Stack }
    }
    static mut _util_Quantizer__Array: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Quantizer__Array() -> ::greycat::GcTypeId {
        unsafe { _util_Quantizer__Array }
    }
    static mut _util_Histogram: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Histogram() -> ::greycat::GcTypeId {
        unsafe { _util_Histogram }
    }
    static mut _util_Queue: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Queue() -> ::greycat::GcTypeId {
        unsafe { _util_Queue }
    }
    static mut _util_QuantizerSlotBound: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_QuantizerSlotBound() -> ::greycat::GcTypeId {
        unsafe { _util_QuantizerSlotBound }
    }
    static mut _util_Quantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Quantizer() -> ::greycat::GcTypeId {
        unsafe { _util_Quantizer }
    }
    static mut _util_CustomQuantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_CustomQuantizer() -> ::greycat::GcTypeId {
        unsafe { _util_CustomQuantizer }
    }
    static mut _util_GaussianProfileSlot: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_GaussianProfileSlot() -> ::greycat::GcTypeId {
        unsafe { _util_GaussianProfileSlot }
    }
    static mut _util_QuantizerSlotBound__Array: ::greycat::GcTypeId = ::greycat::GcTypeId(
        0,
    );
    #[inline(always)]
    pub(crate) fn util_QuantizerSlotBound__Array() -> ::greycat::GcTypeId {
        unsafe { _util_QuantizerSlotBound__Array }
    }
    static mut _util_Crypto: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Crypto() -> ::greycat::GcTypeId {
        unsafe { _util_Crypto }
    }
    static mut _util_HistogramBin: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_HistogramBin() -> ::greycat::GcTypeId {
        unsafe { _util_HistogramBin }
    }
    static mut _util_Gaussian: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Gaussian() -> ::greycat::GcTypeId {
        unsafe { _util_Gaussian }
    }
    static mut _util_GaussianProfile: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_GaussianProfile() -> ::greycat::GcTypeId {
        unsafe { _util_GaussianProfile }
    }
    static mut _util_Plot: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Plot() -> ::greycat::GcTypeId {
        unsafe { _util_Plot }
    }
    static mut _util_LogQuantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_LogQuantizer() -> ::greycat::GcTypeId {
        unsafe { _util_LogQuantizer }
    }
    static mut _util_TimeWindow: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_TimeWindow() -> ::greycat::GcTypeId {
        unsafe { _util_TimeWindow }
    }
    static mut _util_MultiQuantizer: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_MultiQuantizer() -> ::greycat::GcTypeId {
        unsafe { _util_MultiQuantizer }
    }
    static mut _util_Assert: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_Assert() -> ::greycat::GcTypeId {
        unsafe { _util_Assert }
    }
    static mut _util_HistogramStats: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn util_HistogramStats() -> ::greycat::GcTypeId {
        unsafe { _util_HistogramStats }
    }
    static mut _project_Record: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn project_Record() -> ::greycat::GcTypeId {
        unsafe { _project_Record }
    }
    static mut _project_Root: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn project_Root() -> ::greycat::GcTypeId {
        unsafe { _project_Root }
    }
    static mut _csv2_CsvReader__Record: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn csv2_CsvReader__Record() -> ::greycat::GcTypeId {
        unsafe { _csv2_CsvReader__Record }
    }
    static mut _csv2_CsvOptions: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn csv2_CsvOptions() -> ::greycat::GcTypeId {
        unsafe { _csv2_CsvOptions }
    }
    static mut _csv2_CsvReader: ::greycat::GcTypeId = ::greycat::GcTypeId(0);
    #[inline(always)]
    pub(crate) fn csv2_CsvReader() -> ::greycat::GcTypeId {
        unsafe { _csv2_CsvReader }
    }
    pub(crate) const csv2_CsvTrim_Headers: u32 = 0;
    pub(crate) const csv2_CsvTrim_Fields: u32 = 1;
    pub(crate) const csv2_CsvTrim_All: u32 = 2;
    pub(crate) const csv2_CsvOptions_has_header: u32 = 0;
    pub(crate) const csv2_CsvOptions_delimiter: u32 = 1;
    pub(crate) const csv2_CsvOptions_flexible: u32 = 2;
    pub(crate) const csv2_CsvOptions_trim: u32 = 3;
    pub(crate) const csv2_CsvReader_path: u32 = 0;
    pub(crate) const csv2_CsvReader_options: u32 = 1;
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn gc_lib_csv2__link(
        prog: *mut ::greycat::sys::gc_program_t,
        lib: *mut ::greycat::sys::gc_program_library_t,
    ) -> bool {
        unsafe {
            ::greycat::sys::gc_program_library__set_lib_hooks(
                lib,
                Some(csv2_start),
                Some(csv2_stop),
            );
        }
        csv2_link(::greycat::GcProgramMut(prog)).is_some()
    }
    unsafe extern "C" fn csv2_start(
        _lib: *mut ::greycat::sys::gc_program_library_t,
        prog: *mut ::greycat::sys::gc_program_t,
        userdata: *mut *mut ::core::ffi::c_void,
    ) -> bool {
        use ::greycat::GcLibrary as _;
        let prog = ::greycat::GcProgramMut(prog);
        match crate::Library::start(prog) {
            Ok(Some(data)) => {
                let data = Box::new(data);
                unsafe { *userdata = Box::into_raw(data) as *mut _ };
                true
            }
            Ok(None) => true,
            Err(_) => false,
        }
    }
    unsafe extern "C" fn csv2_stop(
        _lib: *mut ::greycat::sys::gc_program_library_t,
        prog: *mut ::greycat::sys::gc_program_t,
        userdata: *mut *mut ::core::ffi::c_void,
    ) -> bool {
        use ::greycat::GcLibrary as _;
        let prog = ::greycat::GcProgramMut(prog);
        let state = unsafe { crate::Library::load(userdata) };
        crate::Library::stop(prog, state).is_ok()
    }
    #[rustfmt::skip]
    fn csv2_link(__prg: ::greycat::GcProgramMut) -> Option<()> {
        unsafe {
            let core = __prg.resolve_module("core")?;
            _core_GeoCircle = __prg.resolve_type(core, "GeoCircle")?;
            _core_Tuple__int_any_null = __prg
                .resolve_type(core, "Tuple<core::int,core::any?>")?;
            _core_VectorIndex = __prg.resolve_type(core, "VectorIndex")?;
            _core_Array__Role = __prg.resolve_type(core, "Array<runtime::Role>")?;
            _core_str = __prg.resolve_type(core, "str")?;
            _core_Array__Permission = __prg
                .resolve_type(core, "Array<runtime::Permission>")?;
            _core_Array__Frame = __prg.resolve_type(core, "Array<runtime::Frame>")?;
            _core_Map__String_MediaTypeObject = __prg
                .resolve_type(core, "Map<core::String,runtime::MediaTypeObject>")?;
            _core_Array__ErrorFrame = __prg
                .resolve_type(core, "Array<core::ErrorFrame>")?;
            _core_float = __prg.resolve_type(core, "float")?;
            _core_t3f = __prg.resolve_type(core, "t3f")?;
            _core_GeoBox = __prg.resolve_type(core, "GeoBox")?;
            _core_Array__ResponseObject = __prg
                .resolve_type(core, "Array<runtime::ResponseObject>")?;
            _core_Array__PathItemObject = __prg
                .resolve_type(core, "Array<runtime::PathItemObject>")?;
            _core_nodeTime = __prg.resolve_type(core, "nodeTime")?;
            _core_Table__GaussianProfileSlot_null = __prg
                .resolve_type(core, "Table<util::GaussianProfileSlot?>")?;
            _core_nodeIndexBucket = __prg.resolve_type(core, "nodeIndexBucket")?;
            _core_Array__node_null = __prg.resolve_type(core, "Array<core::node?>")?;
            _core_Array__HistogramBin = __prg
                .resolve_type(core, "Array<util::HistogramBin>")?;
            _core_String = __prg.resolve_type(core, "String")?;
            _core_NodeInfo__int = __prg.resolve_type(core, "NodeInfo<core::int>")?;
            _core_field = __prg.resolve_type(core, "field")?;
            _core_nodeList = __prg.resolve_type(core, "nodeList")?;
            _core_t3 = __prg.resolve_type(core, "t3")?;
            _core_time = __prg.resolve_type(core, "time")?;
            _core_Table__Tuple__float_any_null = __prg
                .resolve_type(core, "Table<core::Tuple<core::float,core::any?>>")?;
            _core_Array__MediaTypeObject = __prg
                .resolve_type(core, "Array<runtime::MediaTypeObject>")?;
            _core_Array__SearchResult = __prg
                .resolve_type(core, "Array<core::SearchResult>")?;
            _core_Tensor = __prg.resolve_type(core, "Tensor")?;
            _core_Array__HeaderObject = __prg
                .resolve_type(core, "Array<runtime::HeaderObject>")?;
            _core_geo = __prg.resolve_type(core, "geo")?;
            _core_Map__String_UserCredential = __prg
                .resolve_type(core, "Map<core::String,runtime::UserCredential>")?;
            _core_NodeInfo__time = __prg.resolve_type(core, "NodeInfo<core::time>")?;
            _core_Buffer = __prg.resolve_type(core, "Buffer")?;
            _core_Date = __prg.resolve_type(core, "Date")?;
            _core_Array__UserGroupPolicy = __prg
                .resolve_type(core, "Array<runtime::UserGroupPolicy>")?;
            _core_ErrorFrame = __prg.resolve_type(core, "ErrorFrame")?;
            _core_TableColumnMapping = __prg.resolve_type(core, "TableColumnMapping")?;
            _core_Map__String_PathItemObject = __prg
                .resolve_type(core, "Map<core::String,runtime::PathItemObject>")?;
            _core_Array__PeriodicTask = __prg
                .resolve_type(core, "Array<runtime::PeriodicTask>")?;
            _core_Tuple__time_any_null = __prg
                .resolve_type(core, "Tuple<core::time,core::any?>")?;
            _core_Array__DateTuple = __prg
                .resolve_type(core, "Array<runtime::DateTuple>")?;
            _core_Array__SchemaObject = __prg
                .resolve_type(core, "Array<runtime::SchemaObject>")?;
            _core_Array__File = __prg.resolve_type(core, "Array<io::File>")?;
            _core_GeoPoly = __prg.resolve_type(core, "GeoPoly")?;
            _core_nodeList__nodeList__VectorLeaf = __prg
                .resolve_type(core, "nodeList<core::nodeList<core::VectorLeaf>>")?;
            _core_null = __prg.resolve_type(core, "null")?;
            _core_type = __prg.resolve_type(core, "type")?;
            _core_t4f = __prg.resolve_type(core, "t4f")?;
            _core_Array__NodeInfo__geo = __prg
                .resolve_type(core, "Array<core::NodeInfo<core::geo>>")?;
            _core_node = __prg.resolve_type(core, "node")?;
            _core_Error = __prg.resolve_type(core, "Error")?;
            _core_Array__NodeInfo = __prg.resolve_type(core, "Array<core::NodeInfo>")?;
            _core_Array__TableColumnMapping = __prg
                .resolve_type(core, "Array<core::TableColumnMapping>")?;
            _core_Array__Job = __prg.resolve_type(core, "Array<runtime::Job>")?;
            _core_Vector = __prg.resolve_type(core, "Vector")?;
            _core_Array__geo = __prg.resolve_type(core, "Array<core::geo>")?;
            _core_Array__String = __prg.resolve_type(core, "Array<core::String>")?;
            _core_Array__any = __prg.resolve_type(core, "Array<core::any>")?;
            _core_nodeIndex = __prg.resolve_type(core, "nodeIndex")?;
            _core_Tuple__int_VectorLeaf = __prg
                .resolve_type(core, "Tuple<core::int,core::VectorLeaf>")?;
            _core_Tuple__float_any_null = __prg
                .resolve_type(core, "Tuple<core::float,core::any?>")?;
            _core_Map__String_HeaderObject = __prg
                .resolve_type(core, "Map<core::String,runtime::HeaderObject>")?;
            _core_NodeInfo__geo = __prg.resolve_type(core, "NodeInfo<core::geo>")?;
            _core_Map__String_SchemaObject = __prg
                .resolve_type(core, "Map<core::String,runtime::SchemaObject>")?;
            _core_Map__String_int = __prg
                .resolve_type(core, "Map<core::String,core::int>")?;
            _core_Array__Quantizer = __prg.resolve_type(core, "Array<util::Quantizer>")?;
            _core_Map__String_ResponseObject = __prg
                .resolve_type(core, "Map<core::String,runtime::ResponseObject>")?;
            _core_Array__Variable = __prg
                .resolve_type(core, "Array<runtime::Variable>")?;
            _core_Tuple__int_nodeList__VectorLeaf = __prg
                .resolve_type(
                    core,
                    "Tuple<core::int,core::nodeList<core::VectorLeaf>>",
                )?;
            _core_Array__CsvColumnStatistics = __prg
                .resolve_type(core, "Array<io::CsvColumnStatistics>")?;
            _core_Table = __prg.resolve_type(core, "Table")?;
            _core_MathConstants = __prg.resolve_type(core, "MathConstants")?;
            _core_Array__Task = __prg.resolve_type(core, "Array<runtime::Task>")?;
            _core_bool = __prg.resolve_type(core, "bool")?;
            _core_Array__nodeTime = __prg.resolve_type(core, "Array<core::nodeTime>")?;
            _core_Array__nodeList = __prg.resolve_type(core, "Array<core::nodeList>")?;
            _core_Array__nodeIndex = __prg.resolve_type(core, "Array<core::nodeIndex>")?;
            _core_Map__any_int = __prg.resolve_type(core, "Map<core::any,core::int>")?;
            _core_Array__SecurityEntity = __prg
                .resolve_type(core, "Array<runtime::SecurityEntity>")?;
            _core_Array__NodeInfo__int = __prg
                .resolve_type(core, "Array<core::NodeInfo<core::int>>")?;
            _core_Array__int_null = __prg.resolve_type(core, "Array<core::int?>")?;
            _core_Array__int = __prg.resolve_type(core, "Array<core::int>")?;
            _core_duration = __prg.resolve_type(core, "duration")?;
            _core_nodeList__VectorLeaf = __prg
                .resolve_type(core, "nodeList<core::VectorLeaf>")?;
            _core_Array = __prg.resolve_type(core, "Array")?;
            _core_Array__DayOfWeek = __prg
                .resolve_type(core, "Array<runtime::DayOfWeek>")?;
            _core_Tuple = __prg.resolve_type(core, "Tuple")?;
            _core_Map = __prg.resolve_type(core, "Map")?;
            _core_nodeGeo = __prg.resolve_type(core, "nodeGeo")?;
            _core_Array__field = __prg.resolve_type(core, "Array<core::field>")?;
            _core_any = __prg.resolve_type(core, "any")?;
            _core_char = __prg.resolve_type(core, "char")?;
            _core_Array__nodeGeo = __prg.resolve_type(core, "Array<core::nodeGeo>")?;
            _core_nodeTimeCursor = __prg.resolve_type(core, "nodeTimeCursor")?;
            _core_Array__UserCredential = __prg
                .resolve_type(core, "Array<runtime::UserCredential>")?;
            _core_int = __prg.resolve_type(core, "int")?;
            _core_Map__String_String = __prg
                .resolve_type(core, "Map<core::String,core::String>")?;
            _core_Table__Tuple__time_any_null = __prg
                .resolve_type(core, "Table<core::Tuple<core::time,core::any?>>")?;
            _core_t2 = __prg.resolve_type(core, "t2")?;
            _core_t2f = __prg.resolve_type(core, "t2f")?;
            _core_SearchResult = __prg.resolve_type(core, "SearchResult")?;
            _core_function = __prg.resolve_type(core, "function")?;
            _core_Array__float = __prg.resolve_type(core, "Array<core::float>")?;
            _core_VectorLeaf = __prg.resolve_type(core, "VectorLeaf")?;
            _core_NodeInfo = __prg.resolve_type(core, "NodeInfo")?;
            _core_Tuple__geo_any_null = __prg
                .resolve_type(core, "Tuple<core::geo,core::any?>")?;
            _core_t4 = __prg.resolve_type(core, "t4")?;
            _core_Array__NodeInfo__time = __prg
                .resolve_type(core, "Array<core::NodeInfo<core::time>>")?;
            let runtime = __prg.resolve_module("runtime")?;
            _runtime_Runtime = __prg.resolve_type(runtime, "Runtime")?;
            _runtime_ChildProcess = __prg.resolve_type(runtime, "ChildProcess")?;
            _runtime_Periodicity = __prg.resolve_type(runtime, "Periodicity")?;
            _runtime_UserGroupPolicy = __prg.resolve_type(runtime, "UserGroupPolicy")?;
            _runtime_Variable = __prg.resolve_type(runtime, "Variable")?;
            _runtime_OpenApiV3 = __prg.resolve_type(runtime, "OpenApiV3")?;
            _runtime_OperationObject = __prg.resolve_type(runtime, "OperationObject")?;
            _runtime_License = __prg.resolve_type(runtime, "License")?;
            _runtime_FixedPeriodicity = __prg.resolve_type(runtime, "FixedPeriodicity")?;
            _runtime_MonthlyPeriodicity = __prg
                .resolve_type(runtime, "MonthlyPeriodicity")?;
            _runtime_RuntimeInfo = __prg.resolve_type(runtime, "RuntimeInfo")?;
            _runtime_LogDataUsage = __prg.resolve_type(runtime, "LogDataUsage")?;
            _runtime_Debug = __prg.resolve_type(runtime, "Debug")?;
            _runtime_Scheduler = __prg.resolve_type(runtime, "Scheduler")?;
            _runtime_SecurityFields = __prg.resolve_type(runtime, "SecurityFields")?;
            _runtime_MediaTypeObject = __prg.resolve_type(runtime, "MediaTypeObject")?;
            _runtime_Log = __prg.resolve_type(runtime, "Log")?;
            _runtime_InfoObject = __prg.resolve_type(runtime, "InfoObject")?;
            _runtime_SecurityPolicy = __prg.resolve_type(runtime, "SecurityPolicy")?;
            _runtime_Task = __prg.resolve_type(runtime, "Task")?;
            _runtime_UserGroup = __prg.resolve_type(runtime, "UserGroup")?;
            _runtime_SchemaObject = __prg.resolve_type(runtime, "SchemaObject")?;
            _runtime_YearlyPeriodicity = __prg
                .resolve_type(runtime, "YearlyPeriodicity")?;
            _runtime_System = __prg.resolve_type(runtime, "System")?;
            _runtime_DateTuple = __prg.resolve_type(runtime, "DateTuple")?;
            _runtime_Role = __prg.resolve_type(runtime, "Role")?;
            _runtime_ResponseObject = __prg.resolve_type(runtime, "ResponseObject")?;
            _runtime_User = __prg.resolve_type(runtime, "User")?;
            _runtime_Job = __prg.resolve_type(runtime, "Job")?;
            _runtime_Permission = __prg.resolve_type(runtime, "Permission")?;
            _runtime_WeeklyPeriodicity = __prg
                .resolve_type(runtime, "WeeklyPeriodicity")?;
            _runtime_ComponentsObject = __prg.resolve_type(runtime, "ComponentsObject")?;
            _runtime_PeriodicTask = __prg.resolve_type(runtime, "PeriodicTask")?;
            _runtime_PeriodicOptions = __prg.resolve_type(runtime, "PeriodicOptions")?;
            _runtime_UserCredential = __prg.resolve_type(runtime, "UserCredential")?;
            _runtime_ChildProcessResult = __prg
                .resolve_type(runtime, "ChildProcessResult")?;
            _runtime_DailyPeriodicity = __prg.resolve_type(runtime, "DailyPeriodicity")?;
            _runtime_Frame = __prg.resolve_type(runtime, "Frame")?;
            _runtime_RequestBodyObject = __prg
                .resolve_type(runtime, "RequestBodyObject")?;
            _runtime_HeaderObject = __prg.resolve_type(runtime, "HeaderObject")?;
            _runtime_OpenApi = __prg.resolve_type(runtime, "OpenApi")?;
            _runtime_PathItemObject = __prg.resolve_type(runtime, "PathItemObject")?;
            _runtime_OpenIDConnect = __prg.resolve_type(runtime, "OpenIDConnect")?;
            _runtime_SecurityEntity = __prg.resolve_type(runtime, "SecurityEntity")?;
            let io = __prg.resolve_module("io")?;
            _io_GcbWriter = __prg.resolve_type(io, "GcbWriter")?;
            _io_HttpResponse = __prg.resolve_type(io, "HttpResponse")?;
            _io_Writer = __prg.resolve_type(io, "Writer")?;
            _io_GcbReader = __prg.resolve_type(io, "GcbReader")?;
            _io_CsvStatistics = __prg.resolve_type(io, "CsvStatistics")?;
            _io_Http = __prg.resolve_type(io, "Http")?;
            _io_Url = __prg.resolve_type(io, "Url")?;
            _io_TextReader = __prg.resolve_type(io, "TextReader")?;
            _io_CsvFormat = __prg.resolve_type(io, "CsvFormat")?;
            _io_JsonWriter = __prg.resolve_type(io, "JsonWriter")?;
            _io_CsvAnalysisConfig = __prg.resolve_type(io, "CsvAnalysisConfig")?;
            _io_HttpRequest = __prg.resolve_type(io, "HttpRequest")?;
            _io_FileWalker = __prg.resolve_type(io, "FileWalker")?;
            _io_Csv = __prg.resolve_type(io, "Csv")?;
            _io_Json = __prg.resolve_type(io, "Json")?;
            _io_Reader = __prg.resolve_type(io, "Reader")?;
            _io_JsonReader = __prg.resolve_type(io, "JsonReader")?;
            _io_CsvColumnStatistics = __prg.resolve_type(io, "CsvColumnStatistics")?;
            _io_File = __prg.resolve_type(io, "File")?;
            _io_Smtp = __prg.resolve_type(io, "Smtp")?;
            _io_Reader__String = __prg.resolve_type(io, "Reader<core::String>")?;
            _io_CsvWriter = __prg.resolve_type(io, "CsvWriter")?;
            _io_CsvSharding = __prg.resolve_type(io, "CsvSharding")?;
            _io_TextWriter = __prg.resolve_type(io, "TextWriter")?;
            _io_Email = __prg.resolve_type(io, "Email")?;
            _io_XmlReader = __prg.resolve_type(io, "XmlReader")?;
            _io_CsvReader = __prg.resolve_type(io, "CsvReader")?;
            let util = __prg.resolve_module("util")?;
            _util_ProgressTracker = __prg.resolve_type(util, "ProgressTracker")?;
            _util_Random = __prg.resolve_type(util, "Random")?;
            _util_SlidingWindow = __prg.resolve_type(util, "SlidingWindow")?;
            _util_LinearQuantizer = __prg.resolve_type(util, "LinearQuantizer")?;
            _util_Stack = __prg.resolve_type(util, "Stack")?;
            _util_Quantizer__Array = __prg.resolve_type(util, "Quantizer<core::Array>")?;
            _util_Histogram = __prg.resolve_type(util, "Histogram")?;
            _util_Queue = __prg.resolve_type(util, "Queue")?;
            _util_QuantizerSlotBound = __prg.resolve_type(util, "QuantizerSlotBound")?;
            _util_Quantizer = __prg.resolve_type(util, "Quantizer")?;
            _util_CustomQuantizer = __prg.resolve_type(util, "CustomQuantizer")?;
            _util_GaussianProfileSlot = __prg.resolve_type(util, "GaussianProfileSlot")?;
            _util_QuantizerSlotBound__Array = __prg
                .resolve_type(util, "QuantizerSlotBound<core::Array>")?;
            _util_Crypto = __prg.resolve_type(util, "Crypto")?;
            _util_HistogramBin = __prg.resolve_type(util, "HistogramBin")?;
            _util_Gaussian = __prg.resolve_type(util, "Gaussian")?;
            _util_GaussianProfile = __prg.resolve_type(util, "GaussianProfile")?;
            _util_Plot = __prg.resolve_type(util, "Plot")?;
            _util_LogQuantizer = __prg.resolve_type(util, "LogQuantizer")?;
            _util_TimeWindow = __prg.resolve_type(util, "TimeWindow")?;
            _util_MultiQuantizer = __prg.resolve_type(util, "MultiQuantizer")?;
            _util_Assert = __prg.resolve_type(util, "Assert")?;
            _util_HistogramStats = __prg.resolve_type(util, "HistogramStats")?;
            let project = __prg.resolve_module("project")?;
            _project_Record = __prg.resolve_type(project, "Record")?;
            _project_Root = __prg.resolve_type(project, "Root")?;
            let csv2 = __prg.resolve_module("csv2")?;
            _csv2_CsvReader__Record = __prg
                .resolve_type(csv2, "CsvReader<project::Record>")?;
            _csv2_CsvOptions = __prg.resolve_type(csv2, "CsvOptions")?;
            _csv2_CsvReader = __prg.resolve_type(csv2, "CsvReader")?;
            __prg.link_type_fn(_csv2_CsvReader, "can_read", csv2_CsvReader__can_read);
            __prg.link_type_fn(_csv2_CsvReader, "read", csv2_CsvReader__read);
        }
        use ::greycat::GcLibrary as _;
        crate::Library::init(__prg);
        Some(())
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn csv2_CsvReader__can_read(
        ctx: *mut ::greycat::sys::gc_machine_t,
    ) {
        let ctx = ::greycat::GcMachine(ctx);
        let this = unsafe { ctx.get_self_mut() };
        match crate::csv2::CsvReader::can_read(this, ctx) {
            Ok(value) => ctx.set_result(value),
            Err(err) => ctx.set_error(err),
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn csv2_CsvReader__read(ctx: *mut ::greycat::sys::gc_machine_t) {
        let ctx = ::greycat::GcMachine(ctx);
        let this = unsafe { ctx.get_self_mut() };
        match crate::csv2::CsvReader::read(this, ctx) {
            Ok(value) => ctx.set_result(value),
            Err(err) => ctx.set_error(err),
        }
    }
}
pub(crate) struct Library;
impl GcLibrary for Library {
    fn init(prog: greycat::GcProgramMut) {
        prog.configure_type::<
                csv2::CsvReader,
            >(gc::csv2_CsvReader(), csv2::_gc_CsvReader_finalize);
    }
}
