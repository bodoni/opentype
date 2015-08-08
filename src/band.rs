use std::io::{Read, Seek, SeekFrom};
use std::mem;

use Result;

pub trait Band {
    fn position(&mut self) -> Result<u64>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn seek(&mut self, u64) -> Result<u64>;
}

macro_rules! want(
    ($read:expr, $count:expr) => (
        if $read != $count {
            return raise!("failed to read as much as needed");
        }
    );
);

impl<T> Band for T where T: Read + Seek {
    fn read_i16(&mut self) -> Result<i16> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        want!(try!(self.read(&mut buffer)), 2);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        want!(try!(self.read(&mut buffer)), 2);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = unsafe { mem::uninitialized() };
        want!(try!(self.read(&mut buffer)), 4);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buffer: [u8; 8] = unsafe { mem::uninitialized() };
        want!(try!(self.read(&mut buffer)), 8);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
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

#[cfg(target_endian = "big")]
#[inline(always)]
fn convert<T>(_: &mut [T]) {
}

#[cfg(target_endian = "little")]
#[inline]
fn convert<T>(data: &mut [T]) {
    let n = data.len();
    for i in 0..(n / 2) {
        data.swap(i, n - i - 1);
    }
}
