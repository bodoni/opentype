use std::mem;

use Result;
use band::{Band, Value};
use primitive::*;

table!(OffsetTable {
    version       (Fixed ),
    numTables     (USHORT),
    searchRange   (USHORT),
    entrySelector (USHORT),
    rangeShift    (USHORT),
});

table!(TableRecord {
    tag      (ULONG),
    checkSum (ULONG),
    offset   (ULONG),
    length   (ULONG),
});

impl TableRecord {
    #[doc(hidden)]
    pub fn check<T, F>(&self, band: &mut T, process: F) -> Result<bool>
        where T: Band, F: Fn(usize, ULONG) -> ULONG
    {
        let length = {
            let size = mem::size_of::<ULONG>();
            ((self.length as usize + size - 1) & !(size - 1)) / size
        };
        band.stay(|band| {
            try!(band.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(Value::read(band))) as u64;
            }
            Ok(self.checkSum == checksum as u32)
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn table_record_check() {
        use std::io::Cursor;
        use table::TableRecord;

        macro_rules! check(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = TableRecord {
                    length: $length,
                    checkSum: $checksum,
                    .. TableRecord::default()
                };
                table.check(&mut reader, |_, chunk| chunk).unwrap()
            })
        );

        assert!(!check!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( check!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
