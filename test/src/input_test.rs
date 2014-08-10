#![feature(macro_rules)]

use std::io;
use opentype::format::{Endian, OffsetTable};

#[path="../support.rs"]
mod support;

#[path="../../src/input.rs"]
mod input;

#[path="../../src/opentype/mod.rs"]
mod opentype;

#[test]
fn test_read() {
    let fixture = support::find_fixture("SourceSerifPro-Regular.otf").unwrap();
    let mut file = io::File::open(&fixture).unwrap();

    let mut table: OffsetTable = input::read(&mut file).unwrap();
    table.with_big_endian();

    assert_eq!(table.tag, opentype::format::CFF_TAG);
    assert_eq!(table.table_count, 12);
    assert_eq!(table.search_range, 8 * 16);
    assert_eq!(table.entry_selector, 3);
    assert_eq!(table.range_shift, table.table_count * 16 - table.search_range);
}
