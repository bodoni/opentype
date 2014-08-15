#![crate_name = "input"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

pub trait Loader {
    fn from(reader: &mut ::std::io::File)
        -> Result<Self, ::std::io::IoError>;
}

#[macro_export]
macro_rules! read_field(
    ($reader:ident, be_u16) => (
        match $reader.read_be_u16() {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    );
    ($reader:ident, be_u32) => (
        match $reader.read_be_u32() {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    );
    ($reader:ident, le_u32) => (
        match $reader.read_le_u32() {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    );
)

#[macro_export]
macro_rules! implement_loader(
    ($subject:ident, $($field:ident as $size:ident),+) => (
        impl input::Loader for $subject {
            fn from(reader: &mut ::std::io::File)
                -> Result<$subject, ::std::io::IoError> {

                Ok($subject {
                    $(
                        $field: read_field!(reader, $size),
                    )+
                })
            }
        }
    )
)

pub fn stringify_le_u32(value: u32) -> Option<String> {
    ::std::io::extensions::u64_to_le_bytes(value as u64, 4,
        |slice| match ::std::str::from_utf8(slice) {
            Some(result) => Some(String::from_str(result)),
            None => None
        }
    )
}

pub fn read_be_u16(reader: &mut ::std::io::Reader, count: u32)
    -> Result<Vec<u16>, ::std::io::IoError> {

    let mut result = Vec::new();

    for _ in range(0, count) {
        match reader.read_be_u16() {
            Ok(value) => result.push(value),
            Err(error) => return Err(error)
        }
    }

    Ok(result)
}
