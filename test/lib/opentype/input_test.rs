#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate opentype;

use support::*;
use opentype::spec;
use opentype::input;

#[test]
fn structure_read_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let table: spec::OffsetTable = spec::read(&mut file).unwrap();

    assert_eq!(table.tag, spec::CFF_FORMAT_TAG);
    assert_eq!(table.numTables, 12);
    assert_eq!(table.searchRange, 8 * 16);
    assert_eq!(table.entrySelector, 3);
    assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
}

#[test]
fn read_be_u32_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");

    match input::read_be_u32(&mut file, 2).unwrap().as_slice() {
        [one, two, .. rest] => {
            assert_eq!(one, 0x4F54544F);
            assert_eq!(two, 0x000C0080);
            assert!(rest.is_empty());
        }
        _ => assert!(false)
    }
}
