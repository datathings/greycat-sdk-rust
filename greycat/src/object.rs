use greycat_sys::*;

use crate::{machine::GcMachine, types, AsGcValue, GcType, GcTypeId, GcValue};

#[repr(transparent)]
pub struct GcObjectOwned(pub(crate) gc_object_t);

impl AsPtr for GcObjectOwned {
    fn as_ptr(&self) -> *const gc_object_t {
        &self.0 as _
    }
}

impl AsPtrMut for GcObjectOwned {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t {
        &mut self.0 as _
    }
}

#[repr(transparent)]
pub struct GcObject(pub(crate) *mut gc_object);

impl GcObject {
    #[inline(always)]
    pub fn gc_type_id(&self) -> GcTypeId {
        GcTypeId(unsafe { *self.0 }.type_id)
    }

    #[inline(always)]
    pub fn marks(&self) -> u32 {
        unsafe { *self.0 }.marks
    }

    pub fn get_type(&self) -> GcType {
        todo!()
    }
}

#[repr(transparent)]
pub struct GcEnum(pub (u32, u32));

impl AsGcValue for GcObject {
    fn to_value(self) -> (gc_slot, gc_type) {
        (gc_slot::object(self.0), gc_type_object)
    }
}

impl FromPtr<gc_object_t> for GcObject {
    unsafe fn from_ptr(ptr: *mut gc_object_t) -> Self {
        Self(ptr)
    }
}

pub trait AsPtr {
    fn as_ptr(&self) -> *const gc_object_t;
}

pub trait AsPtrMut {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t;

    fn as_object(&mut self) -> GcObject {
        GcObject(self.as_ptr_mut())
    }
}

pub trait FromPtr<T> {
    /// # Safety
    /// You must ensure that the target impl of this trait is effectively related to the underlying `*mut gc_object_t`
    ///
    /// eg. calling `GcString::from_ptr(my_ptr)` will always succeed, though YOU must ensure that the pointer is a `gc_core_string_t`
    unsafe fn from_ptr(ptr: *mut T) -> Self;
}

pub trait Object {
    fn finalize(&mut self, ctx: crate::GcMachine) {
        let _ = ctx;
    }
}

impl AsPtr for GcObject {
    fn as_ptr(&self) -> *const gc_object_t {
        self.0 as _
    }
}

impl AsPtrMut for GcObject {
    fn as_ptr_mut(&mut self) -> *mut gc_object_t {
        self.0
    }
}

pub trait ObjectGetAt<T>
where
    Self: AsPtr,
{
    /// # Safety
    /// It is your responsibility to ensure that the object effectively
    /// contains at least `offset` fields and that the return type is the
    /// actual value type
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> T;
}

// #[allow(unused)]
// fn test_what_i_missed<O: Object>(o: O, ctx: GcMachine) {
//     let v0: bool = unsafe { o.get_at(0, ctx) };
//     let v0: i64 = unsafe { o.get_at(0, ctx) };
//     let v0: f64 = unsafe { o.get_at(0, ctx) };
//     let v0: *mut gc_object_t = unsafe { o.get_at(0, ctx) };
//     let v0: crate::GcString = unsafe { o.get_at(0, ctx) };
//     let v0: crate::GcArray = unsafe { o.get_at(0, ctx) };
//     let v0: crate::GcValue = unsafe { o.get_at(0, ctx) };
//     let v0: crate::GcDuration = unsafe { o.get_at(0, ctx) };
//     let v0: crate::GcTime = unsafe { o.get_at(0, ctx) };
// }

macro_rules! impl_object_get_at {
    ($ty:ty, $gc_ty:ident, $field:ident) => {
        impl<O: AsPtr> ObjectGetAt<$ty> for O {
            unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> $ty {
                let mut type_res = gc_type_null;
                let slot =
                    unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
                debug_assert!(type_res == $gc_ty);
                slot.__1.$field as $ty
            }
        }

        impl<O: AsPtr> ObjectGetAt<Option<$ty>> for O {
            unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<$ty> {
                let mut type_res = gc_type_null;
                let slot =
                    unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
                if type_res == gc_type_null {
                    None
                } else {
                    debug_assert!(type_res == $gc_ty);
                    Some(slot.__1.$field as $ty)
                }
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
impl_object_get_at!(bool, gc_type_bool, b);
impl_object_get_at!(*mut gc_object_t, gc_type_object, object);

impl<O: AsPtr> ObjectGetAt<char> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> char {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_char);
        unsafe { slot.as_char() }
    }
}

impl<O: AsPtr> ObjectGetAt<Option<char>> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<char> {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        if type_res == gc_type_null {
            None
        } else {
            debug_assert!(type_res == gc_type_char);
            Some(unsafe { slot.as_char() })
        }
    }
}

impl<O: AsPtr> ObjectGetAt<GcValue> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> GcValue {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        GcValue::from((slot, type_res))
    }
}

impl<O: AsPtr> ObjectGetAt<(gc_slot_t, gc_type_t)> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> (gc_slot_t, gc_type_t) {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        (slot, type_res)
    }
}

impl<O: AsPtr> ObjectGetAt<()> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) {
        let _ = offset;
        let _ = ctx;
    }
}

impl<O: AsPtr> ObjectGetAt<types::GcDuration> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> types::GcDuration {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_duration);
        types::GcDuration(slot.__1.i64_)
    }
}

impl<O: AsPtr> ObjectGetAt<Option<types::GcDuration>> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<types::GcDuration> {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        if type_res == gc_type_null {
            None
        } else {
            debug_assert!(type_res == gc_type_duration);
            Some(types::GcDuration(slot.__1.i64_))
        }
    }
}

impl<O: AsPtr> ObjectGetAt<GcEnum> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> GcEnum {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_static_field);
        GcEnum((slot.__1.tu32.left, slot.__1.tu32.right))
    }
}

impl<O: AsPtr> ObjectGetAt<Option<GcEnum>> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<GcEnum> {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        if type_res == gc_type_null {
            None
        } else {
            debug_assert!(type_res == gc_type_static_field);
            Some(GcEnum((slot.__1.tu32.left, slot.__1.tu32.right)))
        }
    }
}

impl<O: AsPtr> ObjectGetAt<types::GcTime> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> types::GcTime {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_duration);
        types::GcTime(slot.__1.i64_)
    }
}

impl<O: AsPtr> ObjectGetAt<Option<types::GcTime>> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<types::GcTime> {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        if type_res == gc_type_null {
            None
        } else {
            debug_assert!(type_res == gc_type_time);
            Some(types::GcTime(slot.__1.i64_))
        }
    }
}

impl<O: AsPtr, T: FromPtr<gc_object_t>> ObjectGetAt<T> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> T {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        debug_assert!(type_res == gc_type_object);
        T::from_ptr(slot.__1.object)
    }
}

impl<O: AsPtr, T: FromPtr<gc_object_t>> ObjectGetAt<Option<T>> for O {
    unsafe fn get_at(&self, offset: u32, ctx: GcMachine) -> Option<T> {
        let mut type_res = gc_type_null;
        let slot = unsafe { gc_object__get_at(self.as_ptr(), offset, &mut type_res, ctx.0) };
        if type_res == gc_type_null {
            None
        } else {
            debug_assert!(type_res == gc_type_object);
            Some(T::from_ptr(slot.__1.object))
        }
    }
}
