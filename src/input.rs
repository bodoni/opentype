use std::io;
use std::mem;
use std::ptr;

pub fn read<T>(file: &mut io::File) -> Result<T, io::IoError> {
    match file.read_exact(mem::size_of::<T>()) {
        Ok(buffer) => {
            let pointer: *const T = buffer.as_ptr() as *const T;
            Ok(unsafe{ ptr::read(pointer) })
        },
        Err(error) => Err(error)
    }
}
