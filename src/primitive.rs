//! Primitive data types.

#![allow(non_snake_case)]

use std::io::Read;
use std::{mem, ptr};

use Result;
use band::{Band, Value};

pub type BYTE = u8;
pub type CHAR = i8;

pub type USHORT = u16;
pub type SHORT = i16;

pub type UFWORD = USHORT;
pub type FWORD = SHORT;

pub type ULONG = u32;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

pub type LONGDATETIME = i64;

impl Fixed {
    pub fn as_f32(&self) -> f32 {
        ((self.0 as f32) * 0.0000152587890625 * 1000.0).round() / 1000.0
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

implement!(BYTE, 1);
implement!(CHAR, 1);

implement!(USHORT, 2);
implement!(SHORT, 2);

implement!(ULONG, 4);

implement!(Fixed, 4);

implement!(LONGDATETIME, 8);
