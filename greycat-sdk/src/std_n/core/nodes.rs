use serde::Serialize;

use crate::primitive;

macro_rules! create_node {
    ($name:ident, $name_str:expr, $fqn:expr, $header:expr) => {
        #[derive(Serialize, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name(pub u64);

        impl $name {
            pub const TYPE: &'static str = $fqn;

            #[inline]
            pub fn new(value: u64) -> Self {
                Self(value)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:X}", self.0)
            }
        }

        impl crate::serialize::AbiSerialize for $name {
            fn write_to<W: ::std::io::Write>(
                &self,
                writer: &mut W,
                abi: &crate::abi::Abi,
            ) -> anyhow::Result<usize> {
                use byteorder::WriteBytesExt;
                writer.write_u8($header)?;
                let n = self.write_raw_to(writer, abi)?;
                Ok(1 + n)
            }

            fn write_raw_to<W: ::std::io::Write>(
                &self,
                writer: &mut W,
                _abi: &crate::abi::Abi,
            ) -> anyhow::Result<usize> {
                use crate::varint::VarintWrite;
                let n = writer.write_vu64(self.0)?;
                Ok(n)
            }
        }
    };
}

create_node!(Node, "Node", "core::node", primitive::NODE);
create_node!(NodeList, "NodeList", "core::nodeList", primitive::NODE_LIST);
create_node!(
    NodeIndex,
    "NodeIndex",
    "core::nodeIndex",
    primitive::NODE_INDEX
);
create_node!(NodeGeo, "NodeGeo", "core::nodeGeo", primitive::NODE_GEO);
create_node!(NodeTime, "NodeTime", "core::nodeTime", primitive::NODE_TIME);
