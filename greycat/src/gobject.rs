use greycat_sys::*;

use crate::{AsSlot, Machine, Value};

/// Borrowed `gc_object_t` that will not unmark on drop
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GObj(*mut gc_object_t);

impl From<*mut gc_object_t> for GObj {
    #[inline(always)]
    fn from(value: *mut gc_object_t) -> Self {
        Self(value)
    }
}

impl GObj {
    pub fn get(&self, key: u32, ctx: &Machine) -> Value {
        Value::from(self.get_slot(key, ctx))
    }

    #[inline(always)]
    pub fn get_slot(&self, key: u32, ctx: &Machine) -> (gc_slot_t, gc_type_t) {
        let ty: &mut gc_type_t = &mut 0;
        let slot = unsafe { gc_object__get(self.0, key, ty, ctx.0) };
        (slot, *ty)
    }

    #[inline(always)]
    pub fn get_slot_at(&self, offset: u32, ctx: &Machine) -> (gc_slot_t, gc_type_t) {
        let ty: &mut gc_type_t = &mut 0;
        let slot = unsafe { gc_object__get_at(self.0, offset, ty, ctx.0) };
        (slot, *ty)
    }

    pub fn get_at(&self, offset: u32, ctx: &Machine) -> Value {
        Value::from(self.get_slot_at(offset, ctx))
    }

    pub fn set(&mut self, key: u32, value: impl AsSlot, ctx: &Machine) -> bool {
        let (value, ty) = value.as_slot();
        unsafe { gc_object__set(self.0, key, value, ty, ctx.0) }
    }

    pub fn set_at(&self, offset: u32, value: impl AsSlot, ctx: &Machine) -> bool {
        let (value, ty) = value.as_slot();
        unsafe { gc_object__set_at(self.0, offset, value, ty, ctx.0) }
    }
}

/// Owned `gc_object_t` that will unmark on drop
#[derive(Clone, Debug)]
pub struct GObject {
    ptr: *mut gc_object_t,
    ctx: *mut gc_machine_t,
}

impl GObject {
    #[inline(always)]
    pub fn new(ptr: *mut gc_object_t, ctx: *mut gc_machine_t) -> Self {
        Self { ptr, ctx }
    }

    #[inline(always)]
    pub fn get_slot(&self, key: u32) -> (gc_slot_t, gc_type_t) {
        let ty: &mut gc_type_t = &mut 0;
        let slot = unsafe { gc_object__get(self.ptr, key, ty, self.ctx) };
        (slot, *ty)
    }

    #[inline(always)]
    pub fn get_slot_at(&self, offset: u32) -> (gc_slot_t, gc_type_t) {
        let ty: &mut gc_type_t = &mut 0;
        let slot = unsafe { gc_object__get_at(self.ptr, offset, ty, self.ctx) };
        (slot, *ty)
    }

    pub fn get(&self, key: u32) -> Value {
        Value::from(self.get_slot(key))
    }

    pub fn get_at(&self, offset: u32) -> Value {
        Value::from(self.get_slot_at(offset))
    }

    pub fn set(&mut self, key: u32, value: impl AsSlot) -> bool {
        let (value, ty) = value.as_slot();
        unsafe { gc_object__set(self.ptr, key, value, ty, self.ctx) }
    }

    pub fn set_at(&self, offset: u32, value: impl AsSlot) -> bool {
        let (value, ty) = value.as_slot();
        unsafe { gc_object__set_at(self.ptr, offset, value, ty, self.ctx) }
    }
}

impl Drop for GObject {
    fn drop(&mut self) {
        unsafe { gc_object__un_mark(self.ptr, self.ctx) }
    }
}
