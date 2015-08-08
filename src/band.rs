use std::io::{Read, Seek, SeekFrom};
use std::{mem, ptr};

use Result;

pub trait Band {
    fn position(&mut self) -> Result<u64>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn seek(&mut self, u64) -> Result<u64>;
}

#[cfg(target_endian = "big")]
macro_rules! convert(
    ($data:ident) => ();
);

#[cfg(target_endian = "little")]
macro_rules! convert(
    ($data:ident) => ($data.reverse());
);

macro_rules! read(
    ($this:ident, $count:expr) => (unsafe {
        let mut buffer: [u8; $count] = mem::uninitialized();
        if try!($this.read(&mut buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
        convert!(buffer);
        Ok(ptr::read(buffer.as_ptr() as *const _))
    });
);

impl<T> Band for T where T: Read + Seek {
    fn read_i16(&mut self) -> Result<i16> {
        read!(self, 2)
    }

    fn read_u16(&mut self) -> Result<u16> {
        read!(self, 2)
    }

    fn read_u32(&mut self) -> Result<u32> {
        read!(self, 4)
    }

    fn read_i64(&mut self) -> Result<i64> {
        read!(self, 8)
    }

    #[inline]
    fn seek(&mut self, position: u64) -> Result<u64> {
        Seek::seek(self, SeekFrom::Start(position))
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        Seek::seek(self, SeekFrom::Current(0))
    }
}
