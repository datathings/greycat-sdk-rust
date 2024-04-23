#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::mem::MaybeUninit;

pub const gc_type_null: gc_type_t = 0;
pub const gc_type_bool: gc_type_t = 1;
pub const gc_type_char: gc_type_t = 2;
pub const gc_type_int: gc_type_t = 3;
pub const gc_type_float: gc_type_t = 4;
pub const gc_type_node: gc_type_t = 5;
pub const gc_type_node_time: gc_type_t = 6;
pub const gc_type_node_index: gc_type_t = 7;
pub const gc_type_node_list: gc_type_t = 8;
pub const gc_type_node_geo: gc_type_t = 9;
pub const gc_type_geo: gc_type_t = 10;
pub const gc_type_time: gc_type_t = 11;
pub const gc_type_duration: gc_type_t = 12;
pub const gc_type_cubic: gc_type_t = 13;
pub const gc_type_enum: gc_type_t = 14;
pub const gc_type_object: gc_type_t = 15;
pub const gc_type_tu2d: gc_type_t = 16;
pub const gc_type_tu3d: gc_type_t = 17;
pub const gc_type_tu4d: gc_type_t = 18;
pub const gc_type_tu5d: gc_type_t = 19;
pub const gc_type_tu6d: gc_type_t = 20;
pub const gc_type_tu10d: gc_type_t = 21;
pub const gc_type_tuf2d: gc_type_t = 22;
pub const gc_type_tuf3d: gc_type_t = 23;
pub const gc_type_tuf4d: gc_type_t = 24;
pub const gc_type_block_ref: gc_type_t = 25;
pub const gc_type_function: gc_type_t = 26;
pub const gc_type_undefined: gc_type_t = 27;
pub const gc_type_stringlit: gc_type_t = 28;
pub const gc_type_error: gc_type_t = 29;

#[doc = " must fit on 8 bits !"]
pub type gc_type_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_buffer_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_object_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_machine_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_program_compiler_context {
    _unused: [u8; 0],
}
pub type gc_program_compiler_context_t = gc_program_compiler_context;
#[repr(C)]
#[derive(Copy, Clone)]
pub union gc_slot_tuple_t {
    pub u2: [u32; 2],
    pub u4: [u16; 4],
}

impl From<(u32, u32)> for gc_slot_tuple_t {
    #[inline(always)]
    fn from(value: (u32, u32)) -> Self {
        gc_slot_tuple_t {
            u2: [value.0, value.1],
        }
    }
}

impl From<(u16, u16, u16, u16)> for gc_slot_tuple_t {
    #[inline(always)]
    fn from(value: (u16, u16, u16, u16)) -> Self {
        gc_slot_tuple_t {
            u4: [value.0, value.1, value.2, value.3],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gc_slot_t {
    pub b: bool,
    pub byte: [u8; 8],
    pub u32: u32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
    pub tuple: gc_slot_tuple_t,
    pub object: *mut gc_object_t,
}

impl From<bool> for gc_slot_t {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self { b: value }
    }
}

impl From<[u8; 8]> for gc_slot_t {
    #[inline(always)]
    fn from(value: [u8; 8]) -> Self {
        Self { byte: value }
    }
}

impl From<char> for gc_slot_t {
    #[inline(always)]
    fn from(value: char) -> Self {
        Self { u32: value as u32 }
    }
}

impl From<u32> for gc_slot_t {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self { u32: value }
    }
}

impl From<i64> for gc_slot_t {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self { i64: value }
    }
}

impl From<u64> for gc_slot_t {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self { u64: value }
    }
}

impl From<f64> for gc_slot_t {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self { f64: value }
    }
}

impl From<gc_slot_tuple_t> for gc_slot_t {
    #[inline(always)]
    fn from(value: gc_slot_tuple_t) -> Self {
        Self { tuple: value }
    }
}

impl From<(u32, u32)> for gc_slot_t {
    #[inline(always)]
    fn from(value: (u32, u32)) -> Self {
        Self {
            tuple: gc_slot_tuple_t {
                u2: [value.0, value.1],
            },
        }
    }
}

impl From<(u16, u16, u16, u16)> for gc_slot_t {
    #[inline(always)]
    fn from(value: (u16, u16, u16, u16)) -> Self {
        Self {
            tuple: gc_slot_tuple_t {
                u4: [value.0, value.1, value.2, value.3],
            },
        }
    }
}

impl From<*mut gc_object_t> for gc_slot_t {
    #[inline(always)]
    fn from(value: *mut gc_object_t) -> Self {
        Self { object: value }
    }
}

impl From<*mut gc_program_symbol_t> for gc_slot_t {
    #[inline(always)]
    fn from(value: *mut gc_program_symbol_t) -> Self {
        Self {
            object: value as *mut _,
        }
    }
}

impl Default for gc_slot_t {
    #[inline(always)]
    fn default() -> Self {
        Self { u64: 0 }
    }
}

pub type gc_slot_type_t = [u32; 2];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_core_array_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_core_map_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_core_table_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_core_tensor_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn gc_machine__create_array(ctx: *const gc_machine_t) -> *mut gc_core_array_t;
    pub fn gc_machine__create_map(ctx: *const gc_machine_t) -> *mut gc_core_map_t;
    pub fn gc_machine__create_table(ctx: *const gc_machine_t) -> *mut gc_core_table_t;
    pub fn gc_machine__create_tensor(ctx: *const gc_machine_t) -> *mut gc_core_tensor_t;
    pub fn gc_machine__create_object(ctx: *const gc_machine_t, type_id: u32) -> *mut gc_object_t;
    pub fn gc_machine__get_param(ctx: *const gc_machine_t, offset: u32) -> gc_slot_t;
    pub fn gc_machine__get_param_type(ctx: *const gc_machine_t, offset: u32) -> gc_type_t;
    pub fn gc_machine__this(this: *mut gc_machine_t) -> gc_slot_t;
    pub fn gc_machine__set_result(this: *mut gc_machine_t, slot: gc_slot_t, slot_type: gc_type_t);
    pub fn gc_machine__set_runtime_error(
        ctx: *mut gc_machine_t,
        msg: *const ::std::os::raw::c_char,
    );
    pub fn gc_object__get(
        this: *mut gc_object_t,
        key: u32,
        ty: *mut gc_type_t,
        ctx: *const gc_machine_t,
    ) -> gc_slot_t;
    pub fn gc_object__get_at(
        this: *const gc_object_t,
        offset: u32,
        type_res: *mut gc_type_t,
        ctx: *const gc_machine_t,
    ) -> gc_slot_t;
    pub fn gc_object__set(
        this: *mut gc_object_t,
        key: u32,
        value: gc_slot_t,
        ty: gc_type_t,
        ctx: *mut gc_machine_t,
    ) -> bool;
    pub fn gc_object__set_at(
        this: *mut gc_object_t,
        offset: u32,
        value: gc_slot_t,
        value_type: gc_type_t,
        ctx: *mut gc_machine_t,
    ) -> bool;
    pub fn gc_object__un_mark(this: *mut gc_object_t, ctx: *mut gc_machine_t);
    pub fn gc_core_string__create_from(
        str_: *const ::std::os::raw::c_char,
        len: u64,
    ) -> *mut gc_core_string_t;
    pub fn gc_core_string__create_from_buffer(buf: *const gc_buffer_t) -> *mut gc_core_string_t;
    pub fn gc_core_string__buffer(str_: *const gc_core_string_t) -> *const ::std::os::raw::c_char;
    pub fn gc_core_string__size(str_: *const gc_core_string_t) -> u32;
    pub fn gc_machine__get_buffer(ctx: *mut gc_machine_t) -> *mut gc_buffer_t;
    pub fn gc_buffer__clear(this: *mut gc_buffer_t);
    pub fn gc_buffer__add_str(this: *mut gc_buffer_t, c: *const ::std::os::raw::c_char, len: u32);
    pub fn gc_buffer__add_cstr(this: *mut gc_buffer_t, c: *const ::std::os::raw::c_char);
    pub fn gc_buffer__add_char(this: *mut gc_buffer_t, c: ::std::os::raw::c_char);
    pub fn gc_buffer__add_u64(this: *mut gc_buffer_t, i: u64);
    pub fn gc_buffer__add_slot(
        this: *mut gc_buffer_t,
        slot: gc_slot_t,
        ty: gc_type_t,
        ctx: *const gc_machine_t,
    );
    pub fn gc_buffer__prepare(this: *mut gc_buffer_t, needed: u32);
    pub fn gc_buffer__data(this: *mut gc_buffer_t) -> *mut ::std::os::raw::c_char;
    pub fn gc_buffer__size(this: *mut gc_buffer_t) -> u32;
    pub fn gc_buffer__add_u64_inplace(this: *mut gc_buffer_t, i: u64, offset: u32);
    pub fn gc_program_library__configure(
        lib: *mut gc_program_library_t,
        start: gc_lifecycle_function_t,
        stop: gc_lifecycle_function_t,
    );
    pub fn gc_common__parse_number(str_: *const ::std::os::raw::c_char, str_len: *mut u32) -> u64;
    pub fn gc_common__parse_sign_number(
        str_: *const ::std::os::raw::c_char,
        str_len: *mut u32,
    ) -> i64;

    // TODO clean below

    pub fn gc_unserializable__foreach_slots(
        this: *mut gc_object_t,
        callback: gc_object_type_foreach_slots_action_t,
        rollback: gc_object_type_foreach_slots_action_t,
        ctx: *const gc_machine_t,
    ) -> bool;
    pub fn gc_unserializable__save(
        this: *mut gc_object_t,
        buffer: *mut gc_buffer_t,
        prog: *const gc_program_t,
        finalize: bool,
    );
    pub fn gc_unserializable__load(
        s: *mut gc_slot_t,
        owner: *mut gc_block_t,
        buffer: *mut gc_buffer_t,
        registry: *const gc_machine_t,
    ) -> gc_type_t;
    pub fn gc_program__link_type(
        prg: *const gc_program_t,
        ty: u32,
        foreach_slots: gc_object_type_foreach_slots_t,
        load: gc_object_type_load_t,
        save: gc_object_type_save_t,
        create: gc_object_type_create_t,
        finalize: gc_object_type_finalize_t,
        to_string: gc_object_type_to_string_t,
        functions: *const gc_program_function_body_t,
        offsets: *mut u32,
        functions_nb: u32,
    );
    pub fn gc_program__link_mod(
        prg: *const gc_program_t,
        module: u32,
        functions: *const gc_program_function_body_t,
        offsets: *mut u32,
        functions_nb: u32,
    );
    pub fn gc_program__resolve_symbol(
        program: *const gc_program_t,
        str_: *const ::std::os::raw::c_char,
        len: u32,
    ) -> u32;
    pub fn gc_program__resolve_module(program: *const gc_program_t, mod_name_offset: u32) -> u32;
    pub fn gc_program__resolve_type(
        prog: *const gc_program_t,
        mod_offset: u32,
        type_name_off: u32,
    ) -> u32;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_program_symbol_t {
    _unused: [u8; 0],
}

pub type gc_core_string_t = gc_program_symbol_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_program_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_program_library_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_program_type_t {
    _unused: [u8; 0],
}

pub type gc_lifecycle_function_t = Option<extern "C" fn() -> bool>;
pub type gc_program_function_body_t = Option<extern "C" fn(ctx: *mut gc_machine_t)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_block_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gc_object_type_t {
    _unused: [u8; 0],
}
pub type gc_object_type_foreach_slots_action_t = Option<
    extern "C" fn(
        parent: *mut gc_object_t,
        slot: gc_slot_t,
        slot_type: gc_type_t,
        ctx: *const gc_machine_t,
    ) -> bool,
>;
pub type gc_object_type_foreach_slots_t = Option<
    extern "C" fn(
        this: *mut gc_object_t,
        callback: gc_object_type_foreach_slots_action_t,
        rollback: gc_object_type_foreach_slots_action_t,
        ctx: *const gc_machine_t,
    ) -> bool,
>;
pub type gc_object_type_finalize_t =
    Option<extern "C" fn(this: *mut gc_object_t, ctx: *mut gc_machine_t)>;
pub type gc_object_type_to_string_t = Option<
    extern "C" fn(this: *const gc_object_t, target: *mut gc_buffer_t, prog: *const gc_program_t),
>;
pub type gc_object_type_load_t = Option<
    extern "C" fn(
        s: *mut gc_slot_t,
        owner: *mut gc_block_t,
        buffer: *mut gc_buffer_t,
        ctx: *const gc_machine_t,
    ) -> gc_type_t,
>;
pub type gc_object_type_save_t = Option<
    extern "C" fn(
        this: *mut gc_object_t,
        buffer: *mut gc_buffer_t,
        prog: *const gc_program_t,
        finalize: bool,
    ),
>;
pub type gc_object_type_create_t = Option<extern "C" fn() -> *mut gc_object_t>;
