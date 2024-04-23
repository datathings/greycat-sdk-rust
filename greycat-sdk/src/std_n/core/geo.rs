use byteorder::{WriteBytesExt, LE};
use morton_encoding::{morton_decode, morton_encode};

use crate::serialize::AbiSerialize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Geo(pub u64);

const LAT_MIN: f64 = -85.05112878;
const LAT_MAX: f64 = 85.05112878;
const EPSILON: f64 = 0.00000001;
const LNG_MIN: f64 = -180.0;
const LNG_MAX: f64 = 180.0;
const STEP_MAX: u64 = 32;

impl Geo {
    pub const TYPE: &'static str = "core::geo";

    pub fn as_lat_lng(&self) -> (f64, f64) {
        let [lng, lat]: [u32; 2] = morton_decode(self.0);
        let lat = LAT_MIN + ((lat as f64 + 0.5) / (1_u64 << STEP_MAX) as f64) * (LAT_MAX - LAT_MIN);
        let lng = LNG_MIN + ((lng as f64 + 0.5) / (1_u64 << STEP_MAX) as f64) * (LNG_MAX - LNG_MIN);
        (lat, lng)
    }

    pub fn from_lat_lng(mut lat: f64, mut lng: f64) -> Self {
        if lat < LAT_MIN {
            lat = LAT_MIN;
        }
        if lat >= LAT_MAX {
            lat = LAT_MAX - EPSILON;
        }
        if lng < LNG_MIN {
            lng = LNG_MIN;
        }
        if lng >= LNG_MAX {
            lng = LNG_MAX - EPSILON;
        }

        let mut lat = (lat - LAT_MIN) / (LAT_MAX - LAT_MIN);
        let mut lng = (lng - LNG_MIN) / (LNG_MAX - LNG_MIN);
        lat *= (1_u64 << STEP_MAX) as f64;
        lng *= (1_u64 << STEP_MAX) as f64;

        let geo: u64 = morton_encode([lng as u32, lat as u32]);
        Self(geo)
    }
}

impl std::fmt::Debug for Geo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (lat, lng) = self.as_lat_lng();
        f.debug_struct("Geo")
            .field("lat", &lat)
            .field("lng", &lng)
            .finish()
    }
}

impl serde::Serialize for Geo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct(Geo::TYPE, 2)?;
        let (lat, lng) = self.as_lat_lng();
        s.serialize_field("lat", &lat)?;
        s.serialize_field("lng", &lng)?;
        s.end()
    }
}

impl AbiSerialize for Geo {
    fn write_to<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        abi: &crate::prelude::Abi,
    ) -> anyhow::Result<usize> {
        writer.write_u8(crate::primitive::GEO)?;
        let n = self.write_raw_to(writer, abi)?;
        Ok(1 + n)
    }

    fn write_raw_to<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        _abi: &crate::prelude::Abi,
    ) -> anyhow::Result<usize> {
        writer.write_u64::<LE>(self.0)?;
        Ok(8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn geo_encode() {
        let lat = 48.1173;
        let lng = -1.6777;
        let geo = Geo::from_lat_lng(lat, lng);
        let (lat2, lng2) = geo.as_lat_lng();
        assert!((lat - lat2).abs() <= 0.00001);
        assert!((lng - lng2).abs() <= 0.00001);
    }
}
