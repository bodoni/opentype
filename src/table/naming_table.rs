use std::mem;

use Result;
use band::Band;
use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamingTable {
    Format0(NamingTable0),
    Format1(NamingTable1),
}

spec! {
    pub NamingTable0 {
        format       (USHORT         ),
        count        (USHORT         ),
        stringOffset (USHORT         ),
        nameRecord   (Vec<NameRecord>) |band, this| { read_vector!(band, this.count) },
        storage      (Vec<u8>        ) |band, this| { this.read_storage(band) },
    }
}

spec! {
    pub NamingTable1 {
        format        (USHORT            ),
        count         (USHORT            ),
        stringOffset  (USHORT            ),
        nameRecord    (Vec<NameRecord>   ) |band, this| { read_vector!(band, this.count) },
        langTagCount  (USHORT            ),
        langTagRecord (Vec<LangTagRecord>) |band, this| { read_vector!(band, this.langTagCount) },
        storage       (Vec<u8>           ) |band, this| { this.read_storage(band) },
    }
}

spec! {
    #[repr(C)]
    #[derive(Copy)]
    pub NameRecord {
        platformID (USHORT),
        encodingID (USHORT),
        languageID (USHORT),
        nameID     (USHORT),
        length     (USHORT),
        offset     (USHORT),
    }
}

spec! {
    #[repr(C)]
    #[derive(Copy)]
    pub LangTagRecord {
        length (USHORT),
        ffset  (USHORT),
    }
}

impl NamingTable0 {
    fn read_storage<T: Band>(&self, band: &mut T) -> Result<Vec<u8>> {
        let current = try!(band.position());
        let above = 3 * mem::size_of::<USHORT>() +
                    self.nameRecord.len() * mem::size_of::<NameRecord>();
        try!(band.jump(current - above as u64 + self.stringOffset as u64));
        read_vector!(band, storage_length(&self.nameRecord))
    }
}

impl NamingTable1 {
    fn read_storage<T: Band>(&self, band: &mut T) -> Result<Vec<u8>> {
        let current = try!(band.position());
        let above = 4 * mem::size_of::<USHORT>() +
                    self.nameRecord.len() * mem::size_of::<NameRecord>() +
                    self.langTagRecord.len() * mem::size_of::<LangTagRecord>();
        try!(band.jump(current - above as u64 + self.stringOffset as u64));
        read_vector!(band, storage_length(&self.nameRecord))
    }
}

fn storage_length(records: &[NameRecord]) -> usize {
    let mut length = 0;
    for record in records {
        let end = record.offset + record.length + 1;
        if end > length {
            length = end;
        }
    }
    length as usize
}
