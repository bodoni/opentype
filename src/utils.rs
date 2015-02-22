use input::Read;
use spec::TableRecord;

pub fn checksum<R, F>(reader: &mut R, record: &TableRecord, process: F) -> bool
    where R: Read, F: Fn(usize, u32) -> u32 {

    use std::mem::size_of;

    let mut checksum: u32 = 0;
    let length = {
        let size = size_of::<u32>();
        ((record.length as usize + size - 1) & !(size - 1)) / size
    };

    for i in range(0, length) {
        match reader.read_u32() {
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

    use input::Reader;
    use spec::TableRecord;

    #[test]
    fn checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                super::checksum(
                    &mut Reader::new(&mut BufReader::new(data)),
                    &TableRecord {
                        length: $length,
                        checkSum: $checksum,
                        .. Default::default()
                    },
                    |_, chunk| chunk,
                )
            })
        );

        assert!(!checksum!(3 * 4,
                           1 + 2 + 4,
                           &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));

        assert!(checksum!(3 * 4,
                          1 + 2 + 3,
                          &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
