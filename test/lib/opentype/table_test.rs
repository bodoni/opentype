#![feature(macro_rules)]

extern crate opentype;

use std::default::Default;
use opentype::Table;
use opentype::spec::TableRecord;

#[test]
fn measure_test() {
    macro_rules! measure(
        ($length:expr) => (
            Table::measure(
                &TableRecord {
                    length: $length,
                    .. Default::default()
                }
            )
        )
    )

    assert_eq!(measure!(20), 5);
    assert_eq!(measure!(21), 6);
}

#[test]
fn check_test() {
    macro_rules! check(
        ($length:expr, $checksum:expr, $data:expr) => (
            Table::check(
                &mut std::io::BufReader::new($data),
                &TableRecord {
                    length: $length,
                    checkSum: $checksum,
                    .. Default::default()
                }
            )
        )
    )

    assert_eq!(check!(3 * 4, 1 + 2 + 4,
        [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]), false);
    assert_eq!(check!(3 * 4, 1 + 2 + 3,
        [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]), true);
}
