//! A parser for OpenType fonts.

#[cfg(test)]
extern crate date;

extern crate num;

use std::io;

macro_rules! raise(
    () => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::Other, "cannot parse the file"));
    );
    ($desc:expr) => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::Other, $desc));
    );
);

pub mod spec;

mod font;
mod input;
mod utils;

pub use font::Font;

pub type Error = io::Error;
pub type Result<T> = io::Result<T>;

#[inline]
pub fn read<R: io::Read + io::Seek>(reader: &mut R) -> Result<Vec<Font>> {
    Ok(vec![try!(font::read(reader))])
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::path::PathBuf;

    pub fn open(name: &str) -> File {
        let path = PathBuf::from("tests").join("fixtures").join(name);
        assert!(fs::metadata(&path).is_ok());
        File::open(&path).unwrap()
    }
}
