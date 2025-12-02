use greycat::{GcDuration, GcSymbolId, GcTime};

use crate::{error::Error, Result};

pub trait GcbRead {
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N]>;
    fn read_into(&mut self, buf: &mut [u8]) -> Result<()>;

    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let [b] = self.read_exact()?;
        Ok(b)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.read_exact()?))
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_exact()?))
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_exact()?))
    }

    #[inline]
    fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.read_exact()?))
    }

    fn read_char(&mut self) -> Result<char> {
        let word: [u8; 8] = self.read_exact()?;
        let bytes: [u8; 4] = [word[0], word[1], word[2], word[3]];
        let char_code = u32::from_le_bytes(bytes);
        char::from_u32(char_code).ok_or(Error::InvalidChar(bytes))
    }

    fn read_vu32(&mut self) -> Result<u32> {
        let mut value = 0u32;

        // Maximum 5 iterations, 7 bits per byte
        for shift in (0..32).step_by(7) {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as u32) << shift;

            if byte < 0x80 {
                return Ok(value);
            }
        }

        // vu32 must not be longer than 5 bytes
        Err(Error::Varint(5))
    }

    fn read_vu64(&mut self) -> Result<u64> {
        let mut value = 0u64;

        // Maximum 10 iterations, 7 bits per byte
        for shift in (0..64).step_by(7) {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as u64) << shift;

            if byte < 0x80 {
                return Ok(value);
            }
        }

        // vu64 must not be longer than 10 bytes
        Err(Error::Varint(10))
    }

    #[inline]
    fn read_vi32(&mut self) -> Result<i32> {
        let zigzag_encoded = self.read_vu32()?;
        let value = ((zigzag_encoded >> 1) as i32) ^ -((zigzag_encoded & 1) as i32);
        Ok(value)
    }

    #[inline]
    fn read_vi64(&mut self) -> Result<i64> {
        let zigzag_encoded = self.read_vu64()?;
        let value = ((zigzag_encoded >> 1) as i64) ^ -((zigzag_encoded & 1) as i64);
        Ok(value)
    }

    #[inline]
    fn read_pf64(&mut self, precision: u8) -> Result<f64> {
        match precision {
            0 => self.read_f64(),
            n => Ok((self.read_vu64()? as f64) * 10f64.powi(n as i32)),
        }
    }

    #[inline]
    fn read_symbol(&mut self) -> Result<GcSymbolId> {
        let id = self.read_u32()?;
        Ok(GcSymbolId(id))
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        let b = self.read_u8()?;
        Ok(b != 0)
    }

    #[inline]
    fn read_int(&mut self) -> Result<i64> {
        self.read_vi64()
    }

    #[inline]
    fn read_float(&mut self) -> Result<f64> {
        self.read_f64()
    }

    #[inline]
    fn read_time(&mut self) -> Result<GcTime> {
        let val = self.read_vi64()?;
        Ok(GcTime(val))
    }

    #[inline]
    fn read_duration(&mut self) -> Result<GcDuration> {
        let val = self.read_vi64()?;
        Ok(GcDuration(val))
    }
}

impl<T: std::io::Read> GcbRead for T {
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut bytes = [0u8; N];
        std::io::Read::read_exact(self, &mut bytes)?;
        Ok(bytes)
    }

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        std::io::Read::read_exact(self, buf)?;
        Ok(())
    }
}

pub fn read_exact<'a, const N: usize>(src: &'a [u8], dst: &mut [u8; N]) -> Result<&'a [u8]> {
    use std::io::{Error as IoError, ErrorKind as IoErrorKind};

    if N > src.len() {
        return Err(Error::Io(IoError::new(
            IoErrorKind::UnexpectedEof,
            "not enough bytes",
        )));
    }

    let (head, tail) = src.split_at(N);
    dst.copy_from_slice(head);
    Ok(tail)
}

pub fn read_into<'a>(src: &'a [u8], dst: &mut [u8]) -> Result<&'a [u8]> {
    use std::io::{Error as IoError, ErrorKind as IoErrorKind};

    if dst.len() > src.len() {
        return Err(Error::Io(IoError::new(
            IoErrorKind::UnexpectedEof,
            "not enough bytes",
        )));
    }

    let (head, tail) = src.split_at(dst.len());
    dst.copy_from_slice(head);
    Ok(tail)
}
