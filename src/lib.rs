//! A parser for OpenType fonts.

#![feature(core, io)]
#![cfg_attr(test, feature(fs, path))]

#[cfg(test)]
#[macro_use]
extern crate assert;

#[cfg(test)]
extern crate date;

use std::io;

macro_rules! raise(
    () => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::Other,
                                         "cannot parse the file", None));
    );
    ($desc:expr) => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::Other, $desc, None));
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
    use std::fs::File;
    use std::path::PathBuf;

    pub fn open(name: &str) -> File {
        use std::fs::PathExt;
        let path = PathBuf::new("tests").join("fixtures").join(name);
        assert!(path.exists());
        File::open(&path).unwrap()
    }
}
