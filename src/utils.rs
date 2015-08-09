use std::mem;

use Result;
use band::{Band, Primitive};
use compound::TableRecord;
use primitive::ULONG;

pub fn checksum<T, F>(band: &mut T, record: &TableRecord, process: F) -> Result<bool>
    where T: Band, F: Fn(usize, ULONG) -> ULONG
{
    let length = {
        let size = mem::size_of::<ULONG>();
        ((record.length as usize + size - 1) & !(size - 1)) / size
    };
    band.save(|band| {
        try!(band.jump(record.offset as u64));
        let mut checksum: u64 = 0;
        for i in 0..length {
            checksum += process(i, try!(ULONG::read(band))) as u64;
        }
        Ok(record.checkSum == checksum as u32)
    })
}

#[cfg(test)]
mod tests {
    use compound::TableRecord;
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
            super::checksum(&mut reader, &table, |_, chunk| chunk).unwrap()
        })
    );

    #[test]
    fn checksum() {
        assert!(!checksum!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( checksum!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
