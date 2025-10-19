use std::ptr::NonNull;

use greycat_sys::*;

use crate::{
    object::AsGcObject,
    types::{GcDuration, GcTime},
};

pub enum GcValue {
    Error,
    Undefined,
    Null,
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64),
    StaticField(u32, u32),
    Node(u64),
    NodeTime(u64),
    NodeIndex(u64),
    NodeList(u64),
    NodeGeo(u64),
    Geo(u64),
    Time(GcTime),
    Duration(GcDuration),
    T2((u32, u32)),
    // gc_type_t3: T3((u21, u21, u21)),
    T4((u16, u16, u16, u16)),
    // gc_type_t2f: T2f((f32, f32)),
    // gc_type_t3f: T3f((u21, u21, u21)),
    // gc_type_t4f: T4f((u16, u16, u16, u16)),
    // gc_type_cubic
    // gc_type_str
    // gc_type_block_ref
    // gc_type_block_inline
    Type(u32),
    Field(u32),
    Function(u32),
    StringLit(u32),
    Object(NonNull<gc_object_t>),
}

impl AsGcValue for GcValue {
    fn as_value(&self) -> (gc_slot, gc_type) {
        match self {
            Self::Undefined => (gc_slot_t::null(), gc_type_undefined),
            Self::Null => (gc_slot_t::null(), gc_type_null),
            Self::Bool(v) => (gc_slot_t::b(*v), gc_type_bool),
            Self::Char(v) => (gc_slot_t::char(*v), gc_type_char),
            Self::Int(v) => (gc_slot_t::i64(*v), gc_type_bool),
            Self::Float(v) => (gc_slot_t::f64(*v), gc_type_float),
            Self::Duration(v) => v.as_value(),
            Self::Time(v) => v.as_value(),
            Self::StaticField(left, right) => {
                (gc_slot_t::tu32(*left, *right), gc_type_static_field)
            }
            Self::Type(v) => (gc_slot_t::u32(*v), gc_type_type),
            Self::Field(v) => (gc_slot_t::u32(*v), gc_type_field),
            Self::Function(v) => (gc_slot_t::u32(*v), gc_type_function),
            Self::StringLit(v) => (gc_slot_t::u32(*v), gc_type_stringlit),
            Self::Object(ptr) => (gc_slot_t::object(ptr.as_ptr()), gc_type_object),
            _ => unimplemented!("todo"),
        }
    }
}

impl From<(gc_slot_t, gc_type_t)> for GcValue {
    fn from(value: (gc_slot_t, gc_type_t)) -> Self {
        #[allow(non_upper_case_globals)]
        match value {
            (_, gc_type_null) => Self::Null,
            (_, gc_type_undefined) => Self::Undefined,
            (_, gc_type_error) => Self::Error,
            (slot, gc_type_bool) => Self::Bool(unsafe { slot.__1.b }),
            (slot, gc_type_int) => Self::Int(unsafe { slot.__1.i64_ }),
            (slot, gc_type_float) => Self::Float(unsafe { slot.__1.f64_ }),
            (slot, gc_type_time) => Self::Time(GcTime(unsafe { slot.__1.i64_ })),
            (slot, gc_type_duration) => Self::Duration(GcDuration(unsafe { slot.__1.i64_ })),
            (slot, gc_type_char) => Self::Char(unsafe { extract_char(&slot.__1) }),
            (slot, gc_type_field) => Self::Field(unsafe { slot.__1.u32_ }),
            (slot, gc_type_function) => Self::Function(unsafe { slot.__1.u32_ }),
            (slot, gc_type_type) => Self::Type(unsafe { slot.__1.u32_ }),
            (slot, gc_type_geo) => Self::Geo(unsafe { slot.__1.u64_ }),
            (slot, gc_type_node) => Self::Node(unsafe { slot.__1.u64_ }),
            (slot, gc_type_node_time) => Self::NodeTime(unsafe { slot.__1.u64_ }),
            (slot, gc_type_node_geo) => Self::NodeGeo(unsafe { slot.__1.u64_ }),
            (slot, gc_type_node_index) => Self::NodeIndex(unsafe { slot.__1.u64_ }),
            (slot, gc_type_node_list) => Self::NodeList(unsafe { slot.__1.u64_ }),
            (slot, gc_type_object) => {
                Self::Object(unsafe { NonNull::new_unchecked(slot.__1.object) })
            }
            // TODO the rest...
            (_, _) => Self::Error,
        }
    }
}

pub trait AsGcValue {
    fn as_value(&self) -> (gc_slot, gc_type);
}

macro_rules! impl_as_value_for_int {
    ($($type:ty),*) => {
        $(
            impl AsGcValue for $type {
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
            impl AsGcValue for $type {
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
            impl AsGcValue for $type {
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

impl AsGcValue for bool {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (
            gc_slot {
                __1: gc_slot__bindgen_ty_1 { b: *self },
            },
            gc_type_bool,
        )
    }
}

impl AsGcValue for () {
    fn as_value(&self) -> (gc_slot, gc_type) {
        (gc_slot::null(), gc_type_null)
    }
}

impl<T: AsGcValue> AsGcValue for Option<T> {
    fn as_value(&self) -> (gc_slot, gc_type) {
        match self {
            Some(value) => value.as_value(),
            None => (gc_slot::null(), gc_type_null),
        }
    }
}

impl<T: AsGcObject> AsGcValue for T {
    fn as_value(&self) -> (gc_slot, gc_type) {
        let object = self.as_object();
        (gc_slot::object(object.0), gc_type_object)
    }
}

impl AsGcValue for String {
    fn as_value(&self) -> (gc_slot, gc_type) {
        let str = self.as_str();
        let ptr = str.as_ptr() as *const _;
        let len = str.len() as u64;
        (
            gc_slot::object(unsafe { gc_core_string__create_from(ptr, len) as _ }),
            gc_type_object,
        )
    }
}

// TODO review this..
unsafe fn extract_char(slot: &gc_slot__bindgen_ty_1) -> char {
    let bytes = &slot.byte;

    // Determine how many bytes the first UTF-8 char uses
    let first_byte = bytes[0];

    let char_len = if first_byte & 0b1000_0000 == 0 {
        1
    } else if first_byte & 0b1110_0000 == 0b1100_0000 {
        2
    } else if first_byte & 0b1111_0000 == 0b1110_0000 {
        3
    } else if first_byte & 0b1111_1000 == 0b1111_0000 {
        4
    } else {
        return '\0';
    };

    // Decode UTF-8
    let slice = &bytes[0..char_len];
    let s = std::str::from_utf8_unchecked(slice);
    s.chars().next().unwrap()
}
