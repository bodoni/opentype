#![crate_name = "input"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

pub trait Structure {
    fn read(stream: &mut ::std::io::File) -> Result<Self, ::std::io::IoError>;
}

#[macro_export]
macro_rules! unwrap_field(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    )
)

#[macro_export]
macro_rules! read_field(
    ($stream:ident, be u16) => (unwrap_field!($stream.read_be_u16()));
    ($stream:ident, be i16) => (unwrap_field!($stream.read_be_i16()));
    ($stream:ident, be u32) => (unwrap_field!($stream.read_be_u32()));
    ($stream:ident, be i64) => (unwrap_field!($stream.read_be_i64()));
    ($stream:ident, be f32) => (unwrap_field!($stream.read_be_f32()));
    ($stream:ident, be f64) => (unwrap_field!($stream.read_be_f64()));
    ($stream:ident, le u32) => (unwrap_field!($stream.read_le_u32()));
)

#[macro_export]
macro_rules! implement_structure(
    ($subject:ident, $($field:ident as $order:ident $size:ident),+) => (
        impl input::Structure for $subject {
            fn read(stream: &mut ::std::io::File)
                -> Result<$subject, ::std::io::IoError> {

                Ok($subject {
                    $(
                        $field: read_field!(stream, $order $size),
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

pub fn read_be_u32(stream: &mut ::std::io::File, count: uint)
    -> Result<Vec<u32>, ::std::io::IoError> {

    let mut result: Vec<u32> = Vec::new();

    for _ in range(0, count) {
        match stream.read_be_u32() {
            Ok(value) => result.push(value),
            Err(error) => return Err(error)
        }
    }

    Ok(result)
}
