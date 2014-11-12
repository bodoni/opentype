use std::{io, mem};
use spec::TableRecord;

pub fn checksum<R: io::Reader>(reader: &mut R, record: &TableRecord,
                               process: |uint, u32| -> u32) -> bool {

    let mut checksum: u32 = 0;
    let length = {
        let size = mem::size_of::<u32>();
        ((record.length as uint + size - 1) & !(size - 1)) / size
    };

    for i in range(0, length) {
        match reader.read_be_u32() {
            Ok(chunk) => checksum += process(i, chunk),
            Err(_) => return false
        }
    }

    record.checkSum == checksum
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::io::BufReader;

    use spec::TableRecord;

    #[test]
    fn checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr) => (
                super::checksum(
                    &mut BufReader::new($data),
                    &TableRecord {
                        length: $length,
                        checkSum: $checksum,
                        .. Default::default()
                    },
                    |_, chunk| chunk,
                )
            )
        )

        assert!(!checksum!(3 * 4,
                           1 + 2 + 4,
                           [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));

        assert!(checksum!(3 * 4,
                          1 + 2 + 3,
                          [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
