#![feature(core, io)]
#![cfg_attr(test, feature(path))]

#[cfg(test)]
#[macro_use]
extern crate assert;

#[cfg(test)]
extern crate date;

pub use font::Font;

macro_rules! raise(
    () => (
        return Err(::Error {
            kind: ::std::old_io::OtherIoError,
            desc: "cannot parse the file",
            detail: None,
        });
    );
    ($desc:expr) => (
        return Err(::Error {
            kind: ::std::old_io::OtherIoError,
            desc: $desc,
            detail: None,
        });
    );
);

pub mod spec;

mod font;
mod utils;

pub type Error = std::old_io::IoError;
pub type Result<T> = std::old_io::IoResult<T>;

#[cfg(test)]
mod tests {
    use std::old_io::File;

    pub fn open(name: &str) -> File {
        use std::old_io::fs::PathExtensions;
        let path = Path::new("tests").join_many(&["fixtures", name]);
        assert!(path.exists());
        File::open(&path).unwrap()
    }
}
