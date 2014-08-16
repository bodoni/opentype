#![feature(macro_rules)]

extern crate opentype;

#[cfg(test)]
mod Table {
    use std::default::Default;
    use opentype::Table;
    use opentype::format::TableRecord;

    #[test]
    fn measure_test() {
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
    fn check_test() {
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
}
