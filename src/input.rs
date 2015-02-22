use std::{io, mem};

use Result;

pub trait Read {
    fn read_i16(&mut self) -> Result<i16>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_i64(&mut self) -> Result<i64>;
}

pub trait Seek {
    fn jump(&mut self, position: u64) -> Result<u64>;
    fn position(&mut self) -> Result<u64>;
}

pub struct Reader<'d, D: 'd> {
    driver: &'d mut D,
}

macro_rules! want(
    ($read:expr, $want:expr) => (
        if $read != $want {
            return raise!("not enough data");
        }
    );
);

impl<'d, D> Reader<'d, D> {
    #[inline]
    pub fn new(driver: &'d mut D) -> Reader<'d, D> {
        Reader { driver: driver }
    }
}

impl<'d, D> Read for Reader<'d, D> where D: io::Read {
    fn read_i16(&mut self) -> Result<i16> {
        let mut buffer: [u8; 2] = unsafe { mem::zeroed() };
        want!(try!(self.driver.read(&mut buffer)), 2);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buffer: [u8; 2] = unsafe { mem::zeroed() };
        want!(try!(self.driver.read(&mut buffer)), 2);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = unsafe { mem::zeroed() };
        want!(try!(self.driver.read(&mut buffer)), 4);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buffer: [u8; 8] = unsafe { mem::zeroed() };
        want!(try!(self.driver.read(&mut buffer)), 8);
        convert(&mut buffer);
        Ok(unsafe { *(buffer.as_ptr() as *const _) })
    }
}

impl<'d, D> Seek for Reader<'d, D> where D: io::Seek {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.driver.seek(io::SeekFrom::Start(position))
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.driver.seek(io::SeekFrom::Current(0))
    }
}

#[cfg(target_endian = "big")]
#[inline]
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
