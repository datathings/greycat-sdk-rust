use serde::Serializer;

use crate::abi::AbiType;

pub(crate) fn serialize_type_as_fqn<S>(ty: &AbiType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // TODO maybe we can avoid the alloc here?
    serializer.serialize_str(&ty.fqn())
}

// pub(crate) fn serialize_empty_as_none<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if s.is_empty() {
//         serializer.serialize_none()
//     } else {
//         serializer.serialize_str(s)
//     }
// }
