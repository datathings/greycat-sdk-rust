use crate::*;

#[derive(Clone, Copy)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    Tu2(u32, u32),
    Tu4(u16, u16, u16, u16), // do we really need those tuples? if yes, do we need all of them?
    Obj(*mut gc_object_t),
}

pub trait AsSlot {
    fn as_slot(&self) -> (gc_slot_t, gc_type_t);
}

impl AsSlot for Value {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        self.into()
    }
}

impl<T: AsSlot> AsSlot for Option<T> {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        match self {
            Some(value) => value.as_slot(),
            None => (gc_slot_t::default(), gc_type_null),
        }
    }
}

impl From<Value> for (gc_slot_t, gc_type_t) {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => (gc_slot_t::default(), gc_type_null),
            Value::Int(v) => (v.into(), gc_type_int),
            Value::Float(v) => (v.into(), gc_type_float),
            Value::Char(v) => (v.into(), gc_type_char),
            Value::Bool(v) => (v.into(), gc_type_bool),
            Value::Tu2(a, b) => ((a, b).into(), gc_type_tu2d),
            Value::Tu4(a, b, c, d) => ((a, b, c, d).into(), gc_type_tu4d),
            Value::Obj(ptr) => (gc_slot_t { object: ptr }, gc_type_object),
        }
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(value) => value,
            None => Value::Null,
        }
    }
}

impl From<Option<&Value>> for Value {
    fn from(value: Option<&Value>) -> Self {
        match value {
            Some(value) => *value,
            None => Value::Null,
        }
    }
}

impl From<&Value> for (gc_slot_t, gc_type_t) {
    #[inline(always)]
    fn from(value: &Value) -> Self {
        (*value).into()
    }
}

impl From<(gc_slot_t, gc_type_t)> for Value {
    fn from(value: (gc_slot_t, gc_type_t)) -> Self {
        #[allow(non_upper_case_globals)]
        match value {
            (_, gc_type_null) => Self::Null,
            (slot, gc_type_int) => Self::Int(unsafe { slot.i64 }),
            (slot, gc_type_float) => Self::Float(unsafe { slot.f64 }),
            (slot, gc_type_char) => Self::Char(unsafe { std::char::from_u32_unchecked(slot.u32) }),
            (slot, gc_type_bool) => Self::Bool(unsafe { slot.b }),
            (slot, gc_type_tu2d) => {
                let [a, b] = unsafe { slot.tuple.u2 };
                Self::Tu2(a, b)
            }
            (slot, gc_type_tu4d) => {
                let [a, b, c, d] = unsafe { slot.tuple.u4 };
                Self::Tu4(a, b, c, d)
            }
            (slot, gc_type_object) => Self::Obj(unsafe { slot.object }),
            (_slot, gc_type_node) => todo!(),
            (_slot, gc_type_node_time) => todo!(),
            (_slot, gc_type_node_index) => todo!(),
            (_slot, gc_type_node_list) => todo!(),
            (_slot, gc_type_node_geo) => todo!(),
            (_slot, gc_type_geo) => todo!(),
            (_slot, gc_type_time) => todo!(),
            (_slot, gc_type_duration) => todo!(),
            (_slot, gc_type_cubic) => todo!(),
            (_slot, gc_type_enum) => todo!(),
            (_slot, gc_type_tu3d) => todo!(),
            (_slot, gc_type_tu5d) => todo!(),
            (_slot, gc_type_tu6d) => todo!(),
            (_slot, gc_type_tu10d) => todo!(),
            (_slot, gc_type_tuf2d) => todo!(),
            (_slot, gc_type_tuf3d) => todo!(),
            (_slot, gc_type_tuf4d) => todo!(),
            (_slot, gc_type_block_ref) => todo!(),
            (_slot, gc_type_function) => todo!(),
            (_slot, gc_type_undefined) => todo!(),
            (_slot, gc_type_stringlit) => todo!(),
            (_slot, gc_type_error) => todo!(),
            (_, ty) => panic!("value cannot be converted to type {ty}"),
        }
    }
}

impl From<()> for Value {
    #[inline(always)]
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<char> for Value {
    #[inline(always)]
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl From<bool> for Value {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for Value {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<*mut gc_object_t> for Value {
    #[inline(always)]
    fn from(value: *mut gc_object_t) -> Self {
        Self::Obj(value)
    }
}

impl From<*mut gc_program_symbol_t> for Value {
    #[inline(always)]
    fn from(value: *mut gc_program_symbol_t) -> Self {
        Self::Obj(value as *mut gc_object_t)
    }
}

impl From<(u32, u32)> for Value {
    #[inline(always)]
    fn from((a, b): (u32, u32)) -> Self {
        Self::Tu2(a, b)
    }
}

impl From<(u16, u16, u16, u16)> for Value {
    #[inline(always)]
    fn from((a, b, c, d): (u16, u16, u16, u16)) -> Self {
        Self::Tu4(a, b, c, d)
    }
}

impl AsSlot for () {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        (gc_slot_t::default(), gc_type_null)
    }
}

impl AsSlot for i64 {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_int)
    }
}

impl AsSlot for f64 {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_float)
    }
}

impl AsSlot for bool {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_bool)
    }
}

impl AsSlot for char {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_char)
    }
}

impl AsSlot for (u32, u32) {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_tu2d)
    }
}

impl AsSlot for (u16, u16, u16, u16) {
    #[inline(always)]
    fn as_slot(&self) -> (gc_slot_t, gc_type_t) {
        ((*self).into(), gc_type_tu4d)
    }
}
