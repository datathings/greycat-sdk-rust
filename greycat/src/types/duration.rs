use crate::AsGcValue;

pub struct GcDuration(pub i64);

impl AsGcValue for GcDuration {
    #[inline(always)]
    fn as_value(&self) -> (greycat_sys::gc_slot, greycat_sys::gc_type) {
        (
            greycat_sys::gc_slot::i64(self.0),
            greycat_sys::gc_type_duration,
        )
    }
}
