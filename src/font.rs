use std::io::{Read, Seek};

use Result;
use band::{Band, Value};
use primitive::*;
use table::*;

macro_rules! tag(
    ($value:expr) => (unsafe {
        let mut value: [u8; 4] = ::std::mem::transmute($value);
        value.swap(0, 3);
        value.swap(1, 2);
        value
    })
);

/// A font.
pub struct Font {
    pub offset_table: OffsetTable,
    pub char_mapping: CharMapping,
    pub font_header: FontHeader,
    pub max_profile: MaxProfile,
}

macro_rules! some(
    ($option:expr, $object:expr) => (
        match $option {
            Some(value) => value,
            _ => raise!(concat!($object, " is missing")),
        }
    );
);

impl Font {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        let offset_table = try!(read_offset_table(reader));

        let mut font_header = None;
        let mut max_profile = None;
        let mut char_mapping = None;

        for record in offset_table.records.iter() {
            match &tag!(record.tag) {
                b"cmap" => char_mapping = Some(try!(read_char_mapping(reader, record))),
                b"head" => font_header = Some(try!(read_font_header(reader, record))),
                b"maxp" => max_profile = Some(try!(read_max_profile(reader, record))),
                _ => (),
            }
        }

        Ok(Font {
            offset_table: offset_table,
            font_header: some!(font_header, "the font header"),
            max_profile: some!(max_profile, "the maximum profile"),
            char_mapping: some!(char_mapping, "the character-to-glyph mapping"),
        })
    }
}

fn read_offset_table<T: Band>(band: &mut T) -> Result<OffsetTable> {
    const CFF_TAG: &'static [u8; 4] = b"OTTO";

    let header = try!(OffsetTableHeader::read(band));
    if &tag!(header.version) != CFF_TAG {
        raise!("the format of a font is not supported");
    }
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(OffsetTableRecord::read(band)));
    }

    Ok(OffsetTable { header: header, records: records })
}

fn read_char_mapping<T: Band>(band: &mut T, table: &OffsetTableRecord) -> Result<CharMapping> {
    const VERSION_0_0: USHORT = 0;

    if !try!(table.check(band, |_, chunk| chunk)) {
        raise!("the character-to-glyph mapping is corrupted");
    }

    try!(band.jump(table.offset as u64));
    let header = try!(CharMappingHeader::read(band));
    if header.version != VERSION_0_0 {
        raise!("the format of the character-to-glyph mapping header is not supported");
    }
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(CharMappingRecord::read(band)));
    }
    let mut encodings = vec![];
    for encoding in records.iter() {
        try!(band.jump(table.offset as u64 + encoding.offset as u64));
        encodings.push(match try!(band.peek::<USHORT>()) {
            4 => CharMappingEncoding::Format4(try!(Value::read(band))),
            6 => CharMappingEncoding::Format6(try!(Value::read(band))),
            _ => raise!("the format of a character-to-glyph mapping is not supported"),
        });
    }

    Ok(CharMapping { header: header, records: records, encodings: encodings })
}

fn read_font_header<T: Band>(band: &mut T, record: &OffsetTableRecord) -> Result<FontHeader> {
    const MAGIC_NUMBER: ULONG = 0x5F0F3CF5;
    const VERSION_1_0: Fixed = Fixed(0x00010000);

    if !try!(record.check(band, |i, chunk| if i == 2 { 0 } else { chunk })) {
        raise!("the font header is corrupted");
    }

    try!(band.jump(record.offset as u64));
    let header = try!(FontHeader::read(band));
    if header.version != VERSION_1_0 {
        raise!("the format of the font header is not supported");
    }
    if header.magicNumber != MAGIC_NUMBER {
        raise!("the font header is malformed");
    }

    Ok(header)
}

fn read_max_profile<T: Band>(band: &mut T, record: &OffsetTableRecord) -> Result<MaxProfile> {
    const VERSION_0_5: Fixed = Fixed(0x00005000);
    const VERSION_1_0: Fixed = Fixed(0x00010000);

    if !try!(record.check(band, |_, chunk| chunk)) {
        raise!("the maximum profile is corrupted");
    }

    try!(band.jump(record.offset as u64));
    Ok(match try!(band.peek::<Fixed>()) {
        VERSION_0_5 => MaxProfile::Version05(try!(Value::read(band))),
        VERSION_1_0 => MaxProfile::Version10(try!(Value::read(band))),
        _ => {
            raise!("the format of the maximum profile is not supported");
        },
    })
}
