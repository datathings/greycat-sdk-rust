use crate::AsGcValue;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GcTime(pub i64);

impl AsGcValue for GcTime {
    #[inline(always)]
    fn to_value(self) -> (greycat_sys::gc_slot_t, greycat_sys::gc_type_t) {
        (greycat_sys::gc_slot_t::i64(self.0), greycat_sys::gc_type_time)
    }
}
