use crate::AsGcValue;

#[repr(transparent)]
pub struct GcDuration(pub i64);

impl AsGcValue for GcDuration {
    #[inline(always)]
    fn to_value(self) -> (greycat_sys::gc_slot_t, greycat_sys::gc_type_t) {
        (
            greycat_sys::gc_slot_t::i64(self.0),
            greycat_sys::gc_type_duration,
        )
    }
}
