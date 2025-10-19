use greycat_sys::*;

use crate::{machine::GcMachine, GcValue};

#[repr(transparent)]
pub struct GcObject(pub(crate) gc_object_t);

#[repr(transparent)]
pub struct GcObjectRef(pub(crate) *mut gc_object);

pub trait AsGcObject {
    fn as_object(&self) -> GcObjectRef;
}

pub trait FromPtr<T> {
    /// # Safety
    /// You must ensure that the target impl of this trait is effectively related to the underlying `*mut gc_object_t`
    ///
    /// eg. calling `GcString::from_ptr(my_ptr)` will always succeed, though YOU must ensure that the pointer is a `gc_core_string_t`
    unsafe fn from_ptr(ptr: *mut T) -> Self;
}

pub trait ObjectGetAt<T> {
    /// # Safety
    /// It is your responsibility to ensure that the object effectively
    /// contains at least `offset` fields and that the return type is the
    /// actual value type
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> T;
}

macro_rules! impl_object_ref_get_at {
    ($ty:ty, $gc_ty:ident, $field:ident) => {
        impl ObjectGetAt<$ty> for GcObjectRef {
            unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> $ty {
                let mut type_res = gc_type_null;
                let slot =
                    unsafe { gc_object__get_at(self.0 as *const _, offset, &mut type_res, ctx.0) };
                debug_assert!(type_res == $gc_ty);
                slot.__1.$field as $ty
            }
        }
    };
}

impl_object_ref_get_at!(i8, gc_type_int, i64_);
impl_object_ref_get_at!(i16, gc_type_int, i64_);
impl_object_ref_get_at!(i32, gc_type_int, i64_);
impl_object_ref_get_at!(i64, gc_type_int, i64_);
impl_object_ref_get_at!(u8, gc_type_int, i64_);
impl_object_ref_get_at!(u16, gc_type_int, i64_);
impl_object_ref_get_at!(u32, gc_type_int, i64_);
impl_object_ref_get_at!(u64, gc_type_int, i64_);
impl_object_ref_get_at!(f32, gc_type_float, f64_);
impl_object_ref_get_at!(f64, gc_type_float, f64_);
impl_object_ref_get_at!(*mut gc_object_t, gc_type_object, object);
impl_object_ref_get_at!(bool, gc_type_bool, b);
impl ObjectGetAt<GcValue> for GcObjectRef {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> GcValue {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.0 as *const _, offset, &mut type_res, ctx.0) };
        GcValue::from((slot, type_res))
    }
}
impl ObjectGetAt<(gc_slot_t, gc_type_t)> for GcObjectRef {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> (gc_slot_t, gc_type_t) {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.0 as *const _, offset, &mut type_res, ctx.0) };
        (slot, type_res)
    }
}
impl<T: FromPtr<gc_object_t>> ObjectGetAt<T> for GcObjectRef {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> T {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.0 as *const _, offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_object);
        T::from_ptr(slot.__1.object)
    }
}

macro_rules! impl_object_get_at {
    ($ty:ty, $gc_ty:ident, $field:ident) => {
        impl ObjectGetAt<$ty> for GcObject {
            unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> $ty {
                let mut type_res = gc_type_null;
                let slot =
                    unsafe { gc_object__get_at(&self.0 as *const _, offset, &mut type_res, ctx.0) };
                debug_assert!(type_res == $gc_ty);
                slot.__1.$field as $ty
            }
        }
    };
}

impl_object_get_at!(i8, gc_type_int, i64_);
impl_object_get_at!(i16, gc_type_int, i64_);
impl_object_get_at!(i32, gc_type_int, i64_);
impl_object_get_at!(i64, gc_type_int, i64_);
impl_object_get_at!(u8, gc_type_int, i64_);
impl_object_get_at!(u16, gc_type_int, i64_);
impl_object_get_at!(u32, gc_type_int, i64_);
impl_object_get_at!(u64, gc_type_int, i64_);
impl_object_get_at!(f32, gc_type_float, f64_);
impl_object_get_at!(f64, gc_type_float, f64_);
impl_object_get_at!(*mut gc_object_t, gc_type_object, object);
impl_object_get_at!(bool, gc_type_bool, b);
impl ObjectGetAt<GcValue> for GcObject {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> GcValue {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(&self.0 as *const _, offset, &mut type_res, ctx.0) };
        GcValue::from((slot, type_res))
    }
}
impl ObjectGetAt<(gc_slot_t, gc_type_t)> for GcObject {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> (gc_slot_t, gc_type_t) {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(&self.0 as *const _, offset, &mut type_res, ctx.0) };
        (slot, type_res)
    }
}
impl<T: FromPtr<gc_object_t>> ObjectGetAt<T> for GcObject {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> T {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(&self.0 as *const _, offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_object);
        T::from_ptr(slot.__1.object)
    }
}
