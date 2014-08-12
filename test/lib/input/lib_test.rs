#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate input;
extern crate opentype;

use support::*;
use input::Loader;
use opentype::format::{OffsetTable, CFFFormatTag};

#[test]
fn load_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let table: OffsetTable = Loader::load(&mut file).unwrap();

    assert_eq!(table.tag, CFFFormatTag);
    assert_eq!(table.table_count, 12);
    assert_eq!(table.search_range, 8 * 16);
    assert_eq!(table.entry_selector, 3);
    assert_eq!(table.range_shift, table.table_count * 16 - table.search_range);
}

#[test]
fn stringify_le_u32_test() {
    assert_eq!(input::stringify_le_u32(0x64636261).unwrap().as_slice(), "abcd");
}
