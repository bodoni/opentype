#![crate_name = "input"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

pub trait Loader {
    fn load(reader: &mut ::std::io::Reader)
        -> Result<Self, ::std::io::IoError>;
}

#[macro_export]
macro_rules! load_field(
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
            fn load(reader: &mut ::std::io::Reader)
                -> Result<$subject, ::std::io::IoError> {

                Ok($subject {
                    $(
                        $field: load_field!(reader, $size),
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
