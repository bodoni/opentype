#![feature(globs)]

use std::io;

#[path="../../src/input.rs"]
mod input;

#[path="../support.rs"]
mod support;

#[test]
fn test_read() {
    let fixture = support::find_fixture("SourceSerifPro-Regular.otf").unwrap();
    let mut file = io::File::open(&fixture).unwrap();

    struct Table { version: i32 }
    let table: Table = input::read(&mut file).unwrap();

    assert_eq!(table.version, 0x4F54544Fi32);
}
