#[phase(link, plugin)]
extern crate input;

use self::input::endian::*;

pub static CFF_TAG: u32 = 0x4F54544F;

pub struct OffsetTable {
    pub tag: u32,
    pub table_count: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

impl_endian!(OffsetTable,
    tag as u32,
    table_count as u16,
    search_range as u16,
    entry_selector as u16,
    range_shift as u16
)
