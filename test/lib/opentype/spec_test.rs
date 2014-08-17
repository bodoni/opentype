#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate opentype;

use support::*;
use opentype::spec;

#[test]
fn read_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let table: spec::OffsetTable = spec::read(&mut file).unwrap();

    assert_eq!(table.tag, spec::CFF_FORMAT_TAG);
    assert_eq!(table.numTables, 12);
    assert_eq!(table.searchRange, 8 * 16);
    assert_eq!(table.entrySelector, 3);
    assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
}
