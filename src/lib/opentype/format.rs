#[phase(link, plugin)]
extern crate input;

pub static CFF_FORMAT_TAG: u32 = 0x4F54544F;
pub static FONT_HEADER_TAG: u32 = 0x64616568;

macro_rules! define_struct(
    ($name:ident, $($field:ident as $order:ident $size:ident),+) => (
        #[deriving(Default, Show)]
        pub struct $name { $(pub $field: $size,)+ }
        implement_struct_reader!($name, $($field as $order $size),+)
    )
)

define_struct!(OffsetTable,
    tag           as le u32,
    numTables     as be u16,
    searchRange   as be u16,
    entrySelector as be u16,
    rangeShift    as be u16
)

define_struct!(TableRecord,
    tag      as le u32,
    checkSum as be u32,
    offset   as be u32,
    length   as be u32
)

define_struct!(FontHeader,
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
