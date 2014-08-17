#![macro_escape]

use std::io;

pub trait Structure {
    fn read(stream: &mut io::File) -> Result<Self, io::IoError>;
}

macro_rules! read_field(
    ($stream:ident, be f32) => (try!($stream.read_be_f32()));
    ($stream:ident, be f64) => (try!($stream.read_be_f64()));
    ($stream:ident, be i16) => (try!($stream.read_be_i16()));
    ($stream:ident, be i64) => (try!($stream.read_be_i64()));
    ($stream:ident, be u16) => (try!($stream.read_be_u16()));
    ($stream:ident, be u32) => (try!($stream.read_be_u32()));
    ($stream:ident, le u32) => (try!($stream.read_le_u32()));
)

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

macro_rules! define_structure(
    ($name:ident, $($field:ident as $order:ident $size:ident),+) => (
        #[deriving(Default, Show)]
        pub struct $name { $(pub $field: $size,)+ }
        implement_structure!($name, $($field as $order $size),+)
    )
)

pub fn read_be_u32(stream: &mut io::File, count: uint)
    -> Result<Vec<u32>, io::IoError> {

    let mut result: Vec<u32> = Vec::new();

    for _ in range(0, count) {
        match stream.read_be_u32() {
            Ok(value) => result.push(value),
            Err(error) => return Err(error)
        }
    }

    Ok(result)
}
