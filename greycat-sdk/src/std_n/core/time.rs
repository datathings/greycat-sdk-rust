use serde::Serialize;

macro_rules! create_primitive_i64 {
    ($name:ident, $fqn:expr, $header:expr) => {
        #[derive(Serialize, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        // #[allow(non_camel_case_types)]
        pub struct $name(pub i64);

        impl $name {
            pub const TYPE: &'static str = $fqn;

            #[inline]
            pub fn new(value: i64) -> Self {
                Self(value)
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
                let n = writer.write_vi64(self.0)?;
                Ok(n)
            }
        }
    };
}

create_primitive_i64!(Time, "core::time", crate::primitive::TIME);

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use chrono::{DateTime, Utc};

        match chrono::NaiveDateTime::from_timestamp_micros(self.0) {
            // display the timestamp as an ISO8601
            Some(ndt) => {
                let dt = DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc);
                dt.fmt(f)
            }
            // display the actual value if the timestamp is out-of-range of NaiveDateTime
            None => self.0.fmt(f),
        }
    }
}

create_primitive_i64!(Duration, "core::duration", crate::primitive::DURATION);

impl std::fmt::Debug for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = self.0.wrapping_abs() as u64;
        let d = std::time::Duration::from_micros(d);
        if self.0 < 0 {
            write!(f, "-{d:?}")
        } else {
            d.fmt(f)
        }
    }
}
