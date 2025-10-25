use std::ffi;

use greycat_sys::*;

use crate::{program::GcProgram, value::AsGcValue};
use crate::{types::*, WrapPtr, GcTypeId};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GcMachine(pub *mut gc_machine_t);

impl GcMachine {
    #[inline(always)]
    pub fn get_buffer(&mut self) -> &mut GcBuffer {
        let ptr = unsafe { gc_machine__get_buffer(self.0) };
        unsafe { &mut *(ptr as *mut GcBuffer) }
    }

    pub fn get_program(&self) -> GcProgram {
        GcProgram(unsafe { gc_machine__program(self.0) })
    }

    /// # Safety
    /// It is your responsibility to ensure that the inner context effectively
    /// contains at least `offset` parameters and that the request type is the
    /// actual value type
    pub unsafe fn get_param<T>(&self, offset: u32) -> T
    where
        Self: MachineGetParam<T>,
    {
        unsafe { MachineGetParam::get_param(self, offset) }
    }

    /// If the value is a `gc_type_object` it will be unmarked
    pub fn set_result(&self, value: impl AsGcValue) {
        let (slot_value, slot_type) = value.to_value();
        unsafe {
            gc_machine__set_result(self.0, slot_value, slot_type);
            if slot_type == gc_type_object {
                gc_object__un_mark(slot_value.__1.object, self.0)
            }
        }
    }

    pub fn set_error(&self, err: anyhow::Error) {
        let c_string = ffi::CString::new(err.to_string()).expect("invalid string for C");
        unsafe { gc_machine__set_runtime_error(self.0, c_string.as_ptr()) };
    }

    /// # Safety
    /// It is your responsibility to ensure that the generic type matches with the underlying pointer
    #[inline(always)]
    pub unsafe fn create_object<T>(&self, type_id: GcTypeId) -> *mut T {
        gc_machine__create_object(self.0, type_id.0) as *mut T
    }

    /// # Safety
    /// You need to ensure that the cast is valid
    #[inline(always)]
    pub unsafe fn get_self<T>(&self) -> &T {
        let ptr = gc_machine__this(self.0).__1.object as *mut T;
        &*ptr
    }

    /// # Safety
    /// You need to ensure that the cast is valid
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_self_mut<T>(&self) -> &mut T {
        let ptr = gc_machine__this(self.0).__1.object as *mut T;
        &mut *ptr
    }
}

pub trait MachineGetParam<T> {
    /// # Safety
    /// It is your responsibility to ensure that the inner context effectively
    /// contains at least `offset` parameters and that the request type is the
    /// actual value type
    unsafe fn get_param(&self, offset: u32) -> T;
}

macro_rules! impl_machine_get_param {
    ($ty:ty, $field:ident) => {
        impl MachineGetParam<$ty> for GcMachine {
            unsafe fn get_param(&self, offset: u32) -> $ty {
                let slot = unsafe { gc_machine__get_param(self.0, offset) };
                slot.__1.$field as $ty
            }
        }
    };
}

impl_machine_get_param!(i8, i64_);
impl_machine_get_param!(i16, i64_);
impl_machine_get_param!(i32, i64_);
impl_machine_get_param!(i64, i64_);
impl_machine_get_param!(u8, u32_);
impl_machine_get_param!(u16, u32_);
impl_machine_get_param!(u32, u32_);
impl_machine_get_param!(u64, u64_);
impl_machine_get_param!(f32, f64_);
impl_machine_get_param!(f64, f64_);
impl_machine_get_param!(*mut gc_object_t, object);
impl_machine_get_param!(bool, b);

macro_rules! impl_machine_get_param_byte {
    ($len:expr) => {
        impl MachineGetParam<[u8; $len]> for GcMachine {
            unsafe fn get_param(&self, offset: u32) -> [u8; $len] {
                let slot = unsafe { gc_machine__get_param(self.0, offset) };
                (slot.__1.byte[..$len]).try_into().unwrap()
            }
        }
    };
}

impl_machine_get_param_byte!(1);
impl_machine_get_param_byte!(2);
impl_machine_get_param_byte!(3);
impl_machine_get_param_byte!(4);
impl_machine_get_param_byte!(5);
impl_machine_get_param_byte!(6);
impl_machine_get_param_byte!(7);
impl_machine_get_param_byte!(8);

impl MachineGetParam<char> for GcMachine {
    unsafe fn get_param(&self, offset: u32) -> char {
        unsafe {
            let slot = gc_machine__get_param(self.0, offset);
            let b = slot.__1.byte;
            let bytes = [b[0], b[1], b[2], b[3]];
            char::from_u32_unchecked(u32::from_le_bytes(bytes))
        }
    }
}

impl MachineGetParam<(u32, u32)> for GcMachine {
    unsafe fn get_param(&self, offset: u32) -> (u32, u32) {
        let slot = unsafe { gc_machine__get_param(self.0, offset) };
        (slot.__1.tu32.left, slot.__1.tu32.right)
    }
}

impl<T: WrapPtr<gc_object_t>> MachineGetParam<T> for GcMachine {
    unsafe fn get_param(&self, offset: u32) -> T {
        let slot = unsafe { gc_machine__get_param(self.0, offset) };
        WrapPtr::wrap_ptr(slot.__1.object)
    }
}
