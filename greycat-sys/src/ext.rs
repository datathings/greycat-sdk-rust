use crate::*;

impl gc_slot_t {
    #[inline(always)]
    pub fn null() -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 {
                object: std::ptr::null_mut(),
            },
        }
    }

    #[inline(always)]
    pub fn b(b: bool) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { b },
        }
    }

    #[inline(always)]
    pub fn byte(byte: [u8; 8]) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { byte },
        }
    }

    #[inline(always)]
    pub fn f64(f64: f64) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { f64_: f64 },
        }
    }

    #[inline(always)]
    pub fn char(c: char) -> Self {
        let [b0, b1, b2, b3] = (c as u32).to_le_bytes();
        Self {
            __1: gc_slot__bindgen_ty_1 {
                byte: [b0, b1, b2, b3, 0, 0, 0, 0],
            },
        }
    }

    #[inline(always)]
    pub fn i64(i64: i64) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { i64_: i64 },
        }
    }

    #[inline(always)]
    pub fn object(object: *mut gc_object_t) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { object },
        }
    }

    #[inline(always)]
    pub fn tu32(left: u32, right: u32) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 {
                tu32: gc_slot_tuple_u32 { left, right },
            },
        }
    }

    #[inline(always)]
    pub fn u32(u32: u32) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { u32_: u32 },
        }
    }

    #[inline(always)]
    pub fn u64(u64: u64) -> Self {
        Self {
            __1: gc_slot__bindgen_ty_1 { u64_: u64 },
        }
    }
}

impl std::default::Default for gc_slot_t {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}
