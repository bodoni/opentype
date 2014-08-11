#![crate_name = "input"]
#![crate_type = "rlib"]

#![feature(globs, macro_rules)]

use std::io;
use std::mem;
use std::ptr;

use endian::*;

pub mod endian;

pub fn read<T>(file: &mut io::File) -> Result<T, io::IoError> {
    match file.read_exact(mem::size_of::<T>()) {
        Ok(buffer) => {
            let pointer: *const T = buffer.as_ptr() as *const T;
            Ok(unsafe { ptr::read(pointer) })
        },
        Err(error) => Err(error)
    }
}

pub fn read_big_endian<T:Endian>(file: &mut io::File)
    -> Result<T, io::IoError> {

    match read::<T>(file) {
        Ok(result) => Ok(result.with_big_endian()),
        Err(error) => Err(error)
    }
}

pub fn read_little_endian<T:Endian>(file: &mut io::File)
    -> Result<T, io::IoError> {

    match read::<T>(file) {
        Ok(result) => Ok(result.with_little_endian()),
        Err(error) => Err(error)
    }
}
