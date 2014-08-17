use std::io;

pub static CFF_FORMAT_TAG: u32 = 0x4F54544F;
pub static FONT_HEADER_TAG: u32 = 0x64616568;

trait Spec {
    fn read(stream: &mut io::File) -> Result<Self, io::IoError>;
}

#[inline(always)]
pub fn read<T:Spec>(stream: &mut io::File) -> Result<T, io::IoError> {
    Spec::read(stream)
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

macro_rules! implement_spec(
    ($subject:ident, $($field:ident as $order:ident $size:ident),+) => (
        impl Spec for $subject {
            fn read(stream: &mut ::std::io::File)
                -> Result<$subject, ::std::io::IoError> {

                Ok($subject {
                    $($field: read_field!(stream, $order $size),)+
                })
            }
        }
    )
)

macro_rules! define_spec(
    ($name:ident, $($field:ident as $order:ident $size:ident),+) => (
        #[deriving(Default, Show)]
        pub struct $name {
            $(pub $field: $size,)+
        }

        implement_spec!($name, $($field as $order $size),+)
    )
)

define_spec!(OffsetTable,
    tag           as le u32,
    numTables     as be u16,
    searchRange   as be u16,
    entrySelector as be u16,
    rangeShift    as be u16
)

define_spec!(TableRecord,
    tag      as le u32,
    checkSum as be u32,
    offset   as be u32,
    length   as be u32
)

define_spec!(FontHeader,
    version            as be f32,
    fontRevision       as be f32,
    checkSumAdjustment as be u32,
    magicNumber        as be u32,
    flags              as be u16,
    unitsPerEm         as be u16,
    created            as be i64,
    modified           as be i64,
    xMin               as be i16,
    yMin               as be i16,
    xMax               as be i16,
    yMax               as be i16,
    macStyle           as be u16,
    lowestRecPPEM      as be u16,
    fontDirectionHint  as be i16,
    indexToLocFormat   as be i16,
    glyphDataFormat    as be i16
)
