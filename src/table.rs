use std::{io, mem};
use super::spec;

pub struct Table;

impl Table {
    pub fn measure(table_record: &spec::TableRecord) -> uint {
        let length = table_record.length as uint;
        let size = mem::size_of::<u32>();

        ((length + size - 1) & !(size - 1)) / size
    }

    pub fn map_and_check<R: io::Reader>(stream: &mut R,
        table_record: &spec::TableRecord, process: |u32, uint| -> u32)
        -> bool {

        let mut checksum: u32 = 0;
        let length = Table::measure(table_record);

        for i in range(0, length) {
            match stream.read_be_u32() {
                Ok(value) => checksum += process(value, i),
                Err(_) => return false
            }
        }

        table_record.checkSum == checksum
    }

    pub fn check<R: io::Reader>(stream: &mut R,
        table_record: &spec::TableRecord) -> bool {

        Table::map_and_check(stream, table_record, |chunk, _| chunk)
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use std::io::BufReader;

    use super::Table;

    #[test]
    fn check() {
        macro_rules! check(
            ($length:expr, $checksum:expr, $data:expr) => (
                Table::check(
                    &mut BufReader::new($data),
                    &::spec::TableRecord {
                        length: $length,
                        checkSum: $checksum,
                        .. Default::default()
                    }
                )
            )
        )

        assert_eq!(check!(3 * 4, 1 + 2 + 4, [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]), false);
        assert_eq!(check!(3 * 4, 1 + 2 + 3, [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]), true);
    }

    #[test]
    fn measure() {
        macro_rules! measure(
            ($length:expr) => (
                Table::measure(
                    &::spec::TableRecord {
                        length: $length,
                        .. Default::default()
                    }
                )
            )
        )

        assert_eq!(measure!(20), 5);
        assert_eq!(measure!(21), 6);
    }
}
