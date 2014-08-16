#![feature(macro_rules)]

extern crate opentype;

#[cfg(test)]
mod Table {
    use std::default::Default;
    use opentype::format::{Table, TableRecord};

    #[test]
    fn length_test() {
        macro_rules! table(
            ($length:expr) => (
                Table {
                    record: TableRecord {
                        length: $length, .. Default::default()
                    },
                    content: vec!()
                }
            )
        )

        assert_eq!(table!(20).length(), 10);
        assert_eq!(table!(21).length(), 11);
    }

    #[test]
    fn is_valid_test() {
        macro_rules! table(
            ($length:expr, $checksum:expr, $($arguments:expr),+) => (
                Table {
                    record: TableRecord {
                        length: $length,
                        checksum: $checksum,
                        .. Default::default()
                    },
                    content: vec!($($arguments),+)
                }
            )
        )

        assert_eq!(table!(2 * 3, 1 + 2 + 4, 1, 2, 3).is_valid(), false);
        assert_eq!(table!(2 * 3, 1 + 2 + 4, 1, 2, 4).is_valid(), true);
    }
}
