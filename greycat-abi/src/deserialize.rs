use greycat_gcb::GcbRead;

use crate::Result;

pub trait Deserialize: Sized {
    fn deserialize<D>(de: &mut D) -> Result<Self>
    where
        D: Deserializer;
}

pub trait Deserializer: GcbRead {}

impl<T: GcbRead> Deserializer for T {}

pub trait FromBytes<T> {
    fn from_bytes(bytes: &[u8]) -> Result<T>;
}

pub trait FromVec<T> {
    fn from_vec(vec: Vec<u8>) -> Result<T>;
}

pub trait FromReader<T> {
    fn from_reader<R: std::io::Read>(read: &mut R) -> Result<T>;
}

pub struct BytesDeserializer<'de> {
    bytes: &'de [u8],
}

impl<'de> BytesDeserializer<'de> {
    pub fn from_bytes(value: &'de [u8]) -> Self {
        Self { bytes: value }
    }

    pub fn from_vec(value: &'de Vec<u8>) -> Self {
        Self {
            bytes: value.as_slice(),
        }
    }
}

impl<'de> GcbRead for BytesDeserializer<'de> {
    fn read_exact<const N: usize>(&mut self) -> greycat_gcb::Result<[u8; N]> {
        let mut bytes = [0u8; N];
        self.bytes = greycat_gcb::read_exact(self.bytes, &mut bytes)?;
        Ok(bytes)
    }

    fn read_into(&mut self, buf: &mut [u8]) -> greycat_gcb::Result<()> {
        self.bytes = greycat_gcb::read_into(self.bytes, buf)?;
        Ok(())
    }
}

pub struct ReaderDeserializer<'de> {
    reader: &'de mut dyn std::io::Read,
}

impl<'de> GcbRead for ReaderDeserializer<'de> {
    fn read_exact<const N: usize>(&mut self) -> greycat_gcb::Result<[u8; N]> {
        let mut bytes = [0u8; N];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    fn read_into(&mut self, buf: &mut [u8]) -> greycat_gcb::Result<()> {
        self.reader.read_exact(buf)?;
        Ok(())
    }
}

impl<T: Deserialize> FromBytes<T> for T {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<T> {
        T::deserialize(&mut BytesDeserializer { bytes })
    }
}

impl<T: Deserialize> FromVec<T> for T {
    #[inline]
    fn from_vec(vec: Vec<u8>) -> Result<T> {
        T::deserialize(&mut BytesDeserializer {
            bytes: vec.as_slice(),
        })
    }
}

impl<T: Deserialize> FromReader<T> for T {
    fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<T> {
        T::deserialize(&mut ReaderDeserializer { reader })
    }
}
