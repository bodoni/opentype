use std::mem;

use band::Band;
use spec::TableRecord;

pub fn checksum<T, F>(band: &mut T, record: &TableRecord, process: F) -> bool
    where T: Band, F: Fn(usize, u32) -> u32
{
    let mut checksum: u64 = 0;
    let length = {
        let size = mem::size_of::<u32>();
        ((record.length as usize + size - 1) & !(size - 1)) / size
    };

    for i in 0..length {
        match band.read_u32() {
            Ok(chunk) => checksum += process(i, chunk) as u64,
            Err(_) => return false
        }
    }

    record.checkSum == checksum as u32
}

#[cfg(test)]
mod tests {
    use spec::TableRecord;
    use std::io::Cursor;

    macro_rules! checksum(
        ($length:expr, $checksum:expr, $data:expr) => ({
            let data: &[u8] = $data;
            let mut reader = Cursor::new(data);
            let table = TableRecord {
                length: $length,
                checkSum: $checksum,
                .. TableRecord::default()
            };
            super::checksum(&mut reader, &table, |_, chunk| chunk)
        })
    );

    #[test]
    fn checksum() {
        assert!(!checksum!(3 * 4,
                           1 + 2 + 4,
                           &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));

        assert!(checksum!(3 * 4,
                          1 + 2 + 3,
                          &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
