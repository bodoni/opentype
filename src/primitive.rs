//! Primitive data types.

use std::io::Read;
use std::{mem, ptr};

use Result;
use band::{Band, Value};

pub type Byte = u8;
pub type Char = i8;

pub type UShort = u16;
pub type Short = i16;

pub type UFWord = UShort;
pub type FWord = Short;

pub type ULong = u32;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

pub type LongDateTime = i64;

impl Fixed {
    pub fn as_f32(&self) -> f32 {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (self.0 as f32)
    }
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
    ($band:ident, $count:expr) => (unsafe {
        let mut buffer: [u8; $count] = mem::uninitialized();
        if try!($band.read(&mut buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
        convert!(buffer);
        Ok(ptr::read(buffer.as_ptr() as *const _))
    });
);

macro_rules! implement {
    ($name:ty, $count:expr) => (
        impl Value for $name {
            fn read<T: Band>(band: &mut T) -> Result<Self> {
                read!(band, $count)
            }
        }
    );
}

implement!(Byte, 1);
implement!(Char, 1);

implement!(UShort, 2);
implement!(Short, 2);

implement!(ULong, 4);

implement!(Fixed, 4);

implement!(LongDateTime, 8);
