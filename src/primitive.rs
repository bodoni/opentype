//! Primitive data types.

#![allow(non_snake_case)]

/// A 16-bit unsigned integer.
pub type USHORT = u16;

/// A 16-bit signed integer.
pub type SHORT = i16;

/// A 32-bit unsigned integer.
pub type ULONG = u32;

/// A 32-bit signed fixed-point number.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

/// A date in seconds since January 1, 1904.
pub type LONGDATETIME = i64;

impl Fixed {
    #[cfg(test)]
    pub fn as_f32(&self) -> f32 {
        ((self.0 as f32) * 0.0000152587890625 * 1000.0).round() / 1000.0
    }
}
