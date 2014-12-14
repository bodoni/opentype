#![feature(macro_rules, phase)]

#[cfg(test)] #[phase(plugin)] extern crate assert;
#[cfg(test)] extern crate date;

pub use font::Font;

macro_rules! raise(
    () => (
        return Err(::Error {
            kind: ::std::io::OtherIoError,
            desc: "cannot parse the file",
            detail: None,
        });
    );
    ($desc:expr) => (
        return Err(::Error {
            kind: ::std::io::OtherIoError,
            desc: $desc,
            detail: None,
        });
    );
)

pub mod spec;

mod font;
mod utils;

pub type Error = std::io::IoError;
pub type Result<T> = std::io::IoResult<T>;

#[cfg(test)]
mod tests {
    use std::io::File;

    pub fn open(name: &str) -> File {
        use std::io::fs::PathExtensions;
        let path = Path::new("tests").join_many(&["fixtures", name]);
        assert!(path.exists());
        File::open(&path).unwrap()
    }
}
