#[phase(link, plugin)]
extern crate input;

use std::mem;

pub static CFFFormatTag: u32 = 0x4F54544F;

#[deriving(Default, Show)]
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

#[deriving(Default, Show)]
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

pub struct Table {
    pub record: TableRecord,
    pub content: TableContent,
}

impl Table {
    pub fn length_for(table_record: &TableRecord) -> uint {
        let length = table_record.length as uint;
        let size = mem::size_of::<u16>();

        (length + length % size) / size
    }

    pub fn length(&self) -> uint {
        Table::length_for(&self.record)
    }

    pub fn is_valid(&self) -> bool {
        if self.length() != self.content.len() { return false; }

        let mut checksum: u32 = 0;

        for word in self.content.iter() {
            checksum += *word as u32;
        }

        checksum == self.record.checksum
    }
}
