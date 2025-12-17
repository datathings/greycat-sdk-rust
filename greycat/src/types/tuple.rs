use crate::AsGcValue;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GcT2(pub (u32, u32));

impl AsGcValue for GcT2 {
    #[inline(always)]
    fn to_value(self) -> (greycat_sys::gc_slot_t, greycat_sys::gc_type_t) {
        todo!()
    }
}