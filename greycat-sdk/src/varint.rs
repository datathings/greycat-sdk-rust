use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait VarintRead: std::io::Read {
    /// Reads a varint `u32`
    fn read_vu32(&mut self) -> std::io::Result<u32>;
    /// Reads a varint `i64`
    fn read_vi64(&mut self) -> std::io::Result<i64>;
    /// Reads a varint `U64`
    fn read_vu64(&mut self) -> std::io::Result<u64>;
}

impl<T: std::io::Read> VarintRead for T {
    fn read_vu32(&mut self) -> std::io::Result<u32> {
        let mut value: u32 = 0;
        let mut shift: u32 = 0;

        loop {
            let header = u32::from(self.read_u8()?);
            value |= (header & 0x7F) << shift;
            shift += 7;

            if shift > 28 {
                return Ok(value);
            }

            if (header & 0x80) == 0 {
                return Ok(value);
            }
        }
    }

    fn read_vu64(&mut self) -> std::io::Result<u64> {
        let header = u64::from(self.read_u8()?);
        let mut unpacked = header & 0x7F;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 7;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 14;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 21;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 28;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 35;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 42;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= (header & 0x7F) << 49;
        if (header & 0x80) == 0 {
            return Ok(unpacked);
        }

        let header = u64::from(self.read_u8()?);
        unpacked |= header << 56;
        Ok(unpacked)
    }

    fn read_vi64(&mut self) -> std::io::Result<i64> {
        Ok(zigzag_decode(self.read_vu64()?))
    }
}

pub trait VarintWrite: std::io::Write {
    /// Writes a `u32` as a varint
    fn write_vu32(&mut self, value: u32) -> std::io::Result<usize>;
    /// Writes a `i64` as a varint
    fn write_vi64(&mut self, value: i64) -> std::io::Result<usize>;
    /// Writes a `u64` as a varint
    fn write_vu64(&mut self, value: u64) -> std::io::Result<usize>;
}

impl<T: std::io::Write> VarintWrite for T {
    fn write_vu32(&mut self, mut value: u32) -> std::io::Result<usize> {
        let mut n = 0;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.write_u8(byte)?;
            n += 1;
            if value == 0 {
                break;
            }
        }
        Ok(n)
    }

    fn write_vi64(&mut self, value: i64) -> std::io::Result<usize> {
        self.write_vu64(zigzag_encode(value))
    }

    fn write_vu64(&mut self, mut value: u64) -> std::io::Result<usize> {
        let mut n = 0;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.write_u8(byte)?;
            n += 1;
            if value == 0 {
                break;
            }
        }
        Ok(n)
    }
}

#[inline]
fn zigzag_encode(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

// see: http://stackoverflow.com/a/2211086/56332
// casting required because operations like unary negation
// cannot be performed on unsigned integers
#[inline]
fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}
