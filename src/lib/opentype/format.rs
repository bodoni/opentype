#[phase(link, plugin)]
extern crate input;

pub static CFFFormatTag: u32 = 0x4F54544F;

#[deriving(Show)]
pub struct OffsetTable {
    pub tag: u32,
    pub table_count: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

implement_loader!(OffsetTable,
    tag as le_u32,
    table_count as be_u16,
    search_range as be_u16,
    entry_selector as be_u16,
    range_shift as be_u16
)

#[deriving(Show)]
pub struct TableRecord {
    pub tag: u32,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

implement_loader!(TableRecord,
    tag as le_u32,
    checksum as be_u32,
    offset as be_u32,
    length as be_u32
)

pub type TableContent = Vec<u16>;
