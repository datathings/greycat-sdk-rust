use std::ffi;

pub use greycat_macro::greycat;
pub use greycat_sys2::*;

pub mod sys {
    pub use greycat_sys2::*;
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ModuleId(u32);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SymbolId(u32);

impl SymbolId {
    #[inline(always)]
    pub fn as_string(&self, ctx: Machine) -> String {
        String(unsafe { gc_program__get_symbol(ctx.get_program().0, self.0) })
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TypeId(u32);

#[repr(transparent)]
pub struct Symbol(*mut gc_program_symbol_t);

impl Symbol {
    #[inline(always)]
    pub fn id(&self) -> SymbolId {
        SymbolId(unsafe { gc_program__get_symbol_off(self.0) })
    }
}

impl From<*mut gc_program_symbol_t> for Symbol {
    #[inline(always)]
    fn from(value: *mut gc_program_symbol_t) -> Self {
        Self(value)
    }
}

impl AsObject for Symbol {
    #[inline(always)]
    fn as_object(&self) -> Object {
        Object(self.0 as _)
    }
}

pub type MachineFn = unsafe extern "C" fn(ctx: *mut gc_machine_t);
pub type ObjectFinalizeFn = unsafe extern "C" fn(this: *mut gc_object_t, ctx: *mut gc_machine_t);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Program(*const gc_program_t);

impl Program {
    #[inline(always)]
    pub fn get_symbol(&self, offset: u32) -> Symbol {
        Symbol(unsafe { gc_program__get_symbol(self.0, offset) })
    }

    #[inline(always)]
    pub fn get_symbol_id(&self, symbol: &Symbol) -> SymbolId {
        SymbolId(unsafe { gc_program__get_symbol_off(symbol.0) })
    }

    #[inline(always)]
    pub fn resolve_symbol(&self, text: &str) -> SymbolId {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        assert!(id != 0, "unable to resolve symbol '{text}'");
        SymbolId(id)
    }

    #[inline(always)]
    pub fn resolve_symbol_opt(&self, text: &str) -> Option<SymbolId> {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        if id == 0 {
            None
        } else {
            Some(SymbolId(id))
        }
    }
}

impl From<*const gc_program_t> for Program {
    #[inline(always)]
    fn from(value: *const gc_program_t) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ProgramMut(*mut gc_program_t);

impl From<*mut gc_program_t> for ProgramMut {
    #[inline(always)]
    fn from(value: *mut gc_program_t) -> Self {
        Self(value)
    }
}

impl ProgramMut {
    #[inline(always)]
    pub fn get_symbol(&self, offset: u32) -> Symbol {
        Symbol(unsafe { gc_program__get_symbol(self.0, offset) })
    }

    #[inline(always)]
    pub fn get_symbol_id(&self, symbol: &Symbol) -> SymbolId {
        SymbolId(unsafe { gc_program__get_symbol_off(symbol.0) })
    }

    #[inline(always)]
    pub fn resolve_symbol(&self, text: &str) -> SymbolId {
        let id = unsafe { gc_program__resolve_symbol(self.0, text.as_ptr() as _, text.len() as _) };
        assert!(id != 0, "unable to resolve symbol '{text}'");
        SymbolId(id)
    }

    #[inline(always)]
    pub fn resolve_type(&self, module: ModuleId, type_name: &str) -> TypeId {
        let type_name_off = self.resolve_symbol(type_name);
        let id = unsafe { gc_program__resolve_type(self.0, module.0, type_name_off.0) };
        assert!(
            id != 0,
            "unable to resolve type '{type_name}' in module {module:?}"
        );
        TypeId(id)
    }

    #[inline(always)]
    pub fn resolve_module(&self, name: &str) -> ModuleId {
        let mod_name_offset = self.resolve_symbol(name);
        let id = unsafe { gc_program__resolve_module(self.0, mod_name_offset.0) };
        assert!(id != 0, "unable to resolve module '{name}'");
        ModuleId(id)
    }

    #[inline(always)]
    pub fn link_mod_fn(&self, module: SymbolId, fn_name: SymbolId, fn_impl: MachineFn) -> bool {
        unsafe {
            let module_id = gc_program__resolve_module(self.0, module.0);
            gc_program__link_mod_fn(self.0, module_id, Some(fn_impl), fn_name.0)
        }
    }

    #[inline(always)]
    pub fn link_type_fn(&self, type_id: TypeId, fn_name: &str, function: MachineFn) {
        let fn_id = self.resolve_symbol(fn_name);
        assert!(
            unsafe { gc_program__link_type_fn(self.0, type_id.0, Some(function), fn_id.0) },
            "unable to link function {fn_name} from type {type_id:?}"
        )
    }

    #[inline(always)]
    pub fn configure_type(&self, type_id: TypeId, bytes_size: u32, function: ObjectFinalizeFn) {
        unsafe { gc_program_type__configure(self.0, type_id.0, bytes_size, Some(function)) }
    }
}

pub trait AsValue {
    fn as_value(&self) -> (gc_slot, gc_type);
}

macro_rules! impl_as_value_for_int {
    ($($type:ty),*) => {
        $(
            impl AsValue for $type {
                fn as_value(&self) -> (gc_slot, gc_type) {
                    (
                        gc_slot {
                            __1: gc_slot__bindgen_ty_1 { i64_: *self as i64 },
                        },
                        gc_type_int,
                    )
                }
            }
        )*
    };
}

macro_rules! impl_as_value_for_uint {
    ($($type:ty),*) => {
        $(
            impl AsValue for $type {
                fn as_value(&self) -> (gc_slot, gc_type) {
                    (
                        gc_slot {
                            __1: gc_slot__bindgen_ty_1 { u64_: *self as u64 },
                        },
                        gc_type_int,
                    )
                }
            }
        )*
    };
}

macro_rules! impl_as_value_for_float {
    ($($type:ty),*) => {
        $(
            impl AsValue for $type {
                fn as_value(&self) -> (gc_slot, gc_type) {
                    (
                        gc_slot {
                            __1: gc_slot__bindgen_ty_1 { f64_: *self as f64 },
                        },
                        gc_type_float,
                    )
                }
            }
        )*
    };
}

impl_as_value_for_int!(i16, i32, i64, isize);
impl_as_value_for_uint!(u16, u32, u64, usize);
impl_as_value_for_float!(f32, f64);

impl AsValue for bool {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (
            gc_slot {
                __1: gc_slot__bindgen_ty_1 { b: *self },
            },
            gc_type_bool,
        )
    }
}

impl AsValue for () {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (gc_slot::null(), gc_type_null)
    }
}

impl<T: AsValue> AsValue for Option<T> {
    fn as_value(&self) -> (gc_slot, gc_type) {
        match self {
            Some(value) => value.as_value(),
            None => (gc_slot::null(), gc_type_null),
        }
    }
}

impl<T: AsObject> AsValue for T {
    fn as_value(&self) -> (gc_slot, gc_type) {
        let object = self.as_object();
        (gc_slot::object(object.0), gc_type_object)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Machine(pub *mut gc_machine_t);

impl Machine {
    #[inline(always)]
    pub fn get_buffer(&self) -> Buffer {
        Buffer(unsafe { gc_machine__get_buffer(self.0) })
    }

    pub fn get_program(&self) -> Program {
        Program(unsafe { gc_machine__program(self.0) })
    }

    /// # Safety
    /// It is your responsibility to ensure that something is available at the given offset
    #[inline(always)]
    pub unsafe fn get_param(&self, offset: u32) -> Slot {
        Slot(gc_machine__get_param(self.0, offset))
    }

    /// If the value is a `gc_type_object` it will be unmarked
    pub fn set_result(&self, value: impl AsValue) {
        let (slot_value, slot_type) = value.as_value();
        unsafe {
            gc_machine__set_result(self.0, slot_value, slot_type);
            if slot_type == gc_type_object {
                gc_object__un_mark(slot_value.__1.object, self.0)
            }
        }
    }

    pub fn set_error(&self, message: &str) {
        let c_string = ffi::CString::new(message).expect("invalid string for C");
        unsafe { gc_machine__set_runtime_error(self.0, c_string.as_ptr()) };
    }

    /// Sets the result and unmarks the given object
    #[inline(always)]
    pub fn set_result_object(&self, obj: impl AsObject) {
        let obj = obj.as_object();
        unsafe {
            gc_machine__set_result(self.0, obj.as_slot().0, gc_type_object);
            gc_object__un_mark(obj.0, self.0);
        }
    }

    #[inline(always)]
    pub fn create_object(&self, type_id: u32) -> *mut gc_object_t {
        unsafe { gc_machine__create_object(self.0, type_id) }
    }

    /// # Safety
    /// You need to ensure that the cast is valid
    #[inline(always)]
    pub unsafe fn get_self<T>(&self) -> *mut T {
        gc_machine__this(self.0).__1.object as *mut T
    }
}

impl From<*mut gc_machine_t> for Machine {
    #[inline(always)]
    fn from(value: *mut gc_machine_t) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
pub struct Buffer(*mut gc_buffer_t);

impl From<*mut gc_buffer_t> for Buffer {
    #[inline(always)]
    fn from(value: *mut gc_buffer_t) -> Self {
        Self(value)
    }
}

impl Buffer {
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut gc_buffer_t {
        self.0
    }

    #[inline(always)]
    pub fn clear(&self) -> &Self {
        unsafe { gc_buffer__clear(self.0) }
        self
    }

    #[inline(always)]
    pub fn push_str(&self, str: &str) -> &Self {
        unsafe { gc_buffer__add_str(self.0, str.as_ptr() as *const _, str.len() as u32) }
        self
    }

    #[inline(always)]
    pub fn push_string(&self, str: &String) -> &Self {
        unsafe { gc_buffer__add_str(self.0, str.as_ptr() as *const _, str.len() as u32) }
        self
    }

    /// Creates a new `String` by copying the content of this `Buffer`
    #[inline(always)]
    pub fn to_string(&self) -> String {
        let s = String(unsafe { gc_core_string__create_from_buffer(self.0) });
        self.clear();
        s
    }
}

#[repr(transparent)]
pub struct Slot(gc_slot_t);

impl Slot {
    /// # Safety
    /// It is your responsibility to ensure that the underlying `gc_slot_t` is effectively a `gc_object_t`
    #[inline(always)]
    pub unsafe fn as_object(&self) -> Object {
        Object(self.0.__1.object)
    }
}

impl From<gc_slot_t> for Slot {
    #[inline(always)]
    fn from(value: gc_slot_t) -> Self {
        Self(value)
    }
}

impl From<bool> for Slot {
    fn from(value: bool) -> Self {
        Self(gc_slot_t {
            __1: gc_slot__bindgen_ty_1 { b: value },
        })
    }
}

#[repr(transparent)]
pub struct Object(*mut gc_object_t);

impl Object {
    /// # Safety
    /// This is only safe if the underlying `gc_object_t` is effectively a `gc_core_string_t`
    #[inline(always)]
    pub unsafe fn as_string(&self) -> String {
        String(self.0 as *mut gc_core_string_t)
    }

    #[inline(always)]
    pub fn as_slot(&self) -> Slot {
        Slot(gc_slot_t::object(self.0 as _))
    }

    pub fn get_at(&self, offset: u32, ctx: Machine) -> (gc_slot_t, gc_type_t) {
        let mut slot_type = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.0, offset, &mut slot_type, ctx.0) };
        (slot, slot_type)
    }
}

impl From<*mut gc_object_t> for Object {
    #[inline(always)]
    fn from(value: *mut gc_object_t) -> Self {
        Self(value)
    }
}

impl AsValue for Object {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (gc_slot::object(self.0), gc_type_object)
    }
}

#[repr(transparent)]
pub struct Array(*mut gc_core_array_t);

impl Array {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(ctx: Machine) -> Self {
        let ptr = unsafe {
            // TODO those should be cached
            let prog = ctx.get_program();
            let mod_name = "core";
            let mod_name_offset =
                gc_program__resolve_symbol(prog.0, mod_name.as_ptr() as _, mod_name.len() as _);
            let mod_offset = gc_program__resolve_module(prog.0, mod_name_offset);
            let type_name = "Array<core::any>";
            let type_name_off =
                gc_program__resolve_symbol(prog.0, type_name.as_ptr() as _, type_name.len() as _);
            let object_type_code = gc_program__resolve_type(prog.0, mod_offset, type_name_off);
            ctx.create_object(object_type_code) as _
        };
        Self(ptr)
    }

    pub fn add(&mut self, elem: impl AsValue, ctx: Machine) -> bool {
        let (value, value_type) = elem.as_value();
        let res = unsafe { gc_core_array__add_slot(self.0, value, value_type, ctx.0) };
        if res && value_type == gc_type_object {
            unsafe {
                gc_object__un_mark(value.__1.object, ctx.0);
            }
        }
        res
    }

    pub fn add_str(&mut self, elem: &str, ctx: Machine) -> bool {
        let prog = ctx.get_program();
        let value = match prog.resolve_symbol_opt(elem) {
            Some(symb) => symb.as_string(ctx),
            None => String(unsafe {
                gc_core_string__create_from(elem.as_ptr() as _, elem.len() as u64)
            }),
        };
        self.add(value, ctx)
    }

    pub fn set(&mut self, offset: u32, elem: impl AsValue, ctx: Machine) -> bool {
        let (value, value_type) = elem.as_value();
        unsafe { gc_core_array__set_slot(self.0, offset, value, value_type, ctx.0) }
    }

    pub fn get(&mut self, offset: u32) -> Option<(gc_slot, gc_type)> {
        let mut slot = gc_slot::null();
        let mut slot_type = gc_type_null;
        unsafe {
            if gc_core_array__get_slot(self.0, offset, &mut slot, &mut slot_type) {
                if slot_type == gc_type_null {
                    None
                } else {
                    Some((slot, slot_type))
                }
            } else {
                None
            }
        }
    }

    pub fn size(&mut self) -> u32 {
        unsafe { gc_core_array__size(self.0) }
    }
}

impl AsValue for Array {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (gc_slot::object(self.0 as _), gc_type_object)
    }
}

impl From<*mut gc_core_array_t> for Array {
    #[inline(always)]
    fn from(value: *mut gc_core_array_t) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
pub struct StringBuilder(*mut gc_buffer_t);

impl StringBuilder {
    pub fn from_ctx(ctx: &Machine) -> Self {
        Self(ctx.get_buffer().clear().as_mut_ptr())
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self(unsafe { gc_buffer__create() })
    }

    #[inline(always)]
    pub fn push(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        unsafe {
            gc_buffer__add_str(self.0, text.as_ptr() as _, text.len() as _);
        }
        self
    }

    #[inline(always)]
    pub fn clear(&self) {
        unsafe { gc_buffer__clear(self.0) }
    }

    pub fn build(self) -> String {
        String(unsafe { gc_core_string__create_from_buffer(self.0) })
    }
}

impl std::default::Default for StringBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl From<*mut gc_buffer_t> for StringBuilder {
    #[inline(always)]
    fn from(value: *mut gc_buffer_t) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
pub struct String(*mut gc_core_string_t);

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<&Buffer> for String {
    #[inline(always)]
    fn from(value: &Buffer) -> Self {
        value.to_string()
    }
}

impl From<*mut gc_core_string_t> for String {
    #[inline(always)]
    fn from(value: *mut gc_core_string_t) -> Self {
        Self(value)
    }
}

impl String {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { gc_core_string__buffer(self.0) as _ }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { gc_core_string__size(self.0) as _ }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.as_ptr(), self.len());
            std::str::from_utf8_unchecked(slice)
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AsObject for String {
    #[inline(always)]
    fn as_object(&self) -> Object {
        Object(self.0 as _)
    }
}

pub trait AsObject {
    fn as_object(&self) -> Object;
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        unsafe {
            let ptr = gc_core_string__buffer(self.0) as *const u8;
            let len = gc_core_string__size(self.0) as _;
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}
