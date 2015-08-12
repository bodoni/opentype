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
        format       (UShort         ),
        count        (UShort         ),
        stringOffset (UShort         ),
        nameRecord   (Vec<NameRecord>) |band, this| { read_vector!(band, this.count) },
        storage      (Vec<Byte>      ) |band, this| { this.read_storage(band) },
    }
}

spec! {
    pub NamingTable1 {
        format        (UShort                ),
        count         (UShort                ),
        stringOffset  (UShort                ),
        nameRecord    (Vec<NameRecord>       ) |band, this| { read_vector!(band, this.count) },
        langTagCount  (UShort                ),
        langTagRecord (Vec<LanguageTagRecord>) |band, this| { read_vector!(band,
                                                                           this.langTagCount) },
        storage       (Vec<Byte>             ) |band, this| { this.read_storage(band) },
    }
}

spec! {
    #[repr(C)]
    #[derive(Copy)]
    pub NameRecord {
        platformID (UShort),
        encodingID (UShort),
        languageID (UShort),
        nameID     (UShort),
        length     (UShort),
        offset     (UShort),
    }
}

spec! {
    #[repr(C)]
    #[derive(Copy)]
    pub LanguageTagRecord {
        length (UShort),
        ffset  (UShort),
    }
}

impl NamingTable0 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.nameRecord, &self.storage)
    }

    fn read_storage<T: Band>(&self, band: &mut T) -> Result<Vec<u8>> {
        let current = try!(band.position());
        let above = 3 * mem::size_of::<UShort>() +
                    self.nameRecord.len() * mem::size_of::<NameRecord>();
        try!(band.jump(current - above as u64 + self.stringOffset as u64));
        read_vector!(band, storage_length(&self.nameRecord))
    }
}

impl NamingTable1 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.nameRecord, &self.storage)
    }

    fn read_storage<T: Band>(&self, band: &mut T) -> Result<Vec<u8>> {
        let current = try!(band.position());
        let above = 4 * mem::size_of::<UShort>() +
                    self.nameRecord.len() * mem::size_of::<NameRecord>() +
                    self.langTagRecord.len() * mem::size_of::<LanguageTagRecord>();
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

fn strings(records: &[NameRecord], storage: &[u8]) -> Result<Vec<String>> {
    let mut strings = vec![];
    for record in records {
        let (offset, length) = (record.offset as usize, record.length as usize);
        let bytes = &storage[offset..(offset + length)];
        match record.platformID {
            1 => match decode_macintosh(bytes, record.encodingID) {
                Some(string) => {
                    strings.push(string);
                    continue;
                },
                _ => {},
            },
            _ => {},
        }
        strings.push("<unsupported>".to_string());
    }
    Ok(strings)
}

// The implementation is based on
// https://github.com/nodebox/opentype.js/blob/master/src/types.js#L300
fn decode_macintosh(bytes: &[Byte], encoding: UShort) -> Option<String> {
    const ROMAN: [char; 128] = ['Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
                                'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
                                'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
                                'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥',
                                '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿',
                                '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
                                'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄',
                                '€', '‹', '›', 'ﬁ', 'ﬂ', '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
                                'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
                                'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ'];

    if encoding != 0 {
        return None;
    }

    let table = &ROMAN;

    let mut string = String::new();
    for &byte in bytes {
        if byte <= 0x7F {
            string.push(byte as char);
        } else {
            string.push(table[(byte & 0x7F) as usize]);
        }
    }

    Some(string)
}
