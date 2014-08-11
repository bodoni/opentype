#![feature(globs, phase, macro_rules)]

extern crate input;
extern crate opentype;

#[phase(link, plugin)]
extern crate support;

use opentype::format::{OffsetTable, CFF_TAG};
use support::*;

#[test]
fn test_read_big_endian() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let table = input::read_big_endian::<OffsetTable>(&mut file).unwrap();

    assert_eq!(table.tag, CFF_TAG);
    assert_eq!(table.table_count, 12);
    assert_eq!(table.search_range, 8 * 16);
    assert_eq!(table.entry_selector, 3);
    assert_eq!(table.range_shift, table.table_count * 16 - table.search_range);
}
