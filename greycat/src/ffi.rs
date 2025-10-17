pub trait FromPtr {
    type CType;

    fn from_ptr(ptr: *mut Self::CType) -> Self;
}
