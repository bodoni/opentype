#![feature(macro_rules)]

extern crate opentype;

use std::default::Default;
use opentype::Table;
use opentype::spec::TableRecord;

#[test]
fn table_measure_test() {
    macro_rules! table_record(
        ($length:expr) => (
            TableRecord {
                length: $length, .. Default::default()
            }
        )
    )

    assert_eq!(Table::measure(&table_record!(20)), 5);
    assert_eq!(Table::measure(&table_record!(21)), 6);
}

#[test]
fn table_check_test() {
    macro_rules! table(
        ($checksum:expr, $($arguments:expr),+) => (
            Table {
                checksum: $checksum,
                content: vec!($($arguments),+),
            }
        )
    )

    assert_eq!(table!(1 + 2 + 4, 1, 2, 3).check(), false);
    assert_eq!(table!(1 + 2 + 4, 1, 2, 4).check(), true);
}
