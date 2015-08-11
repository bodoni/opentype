use std::io::{Read, Seek};
use std::mem;

use Result;
use band::{Band, Value};
use primitive::*;
use table::*;

/// A font.
pub struct Font {
    pub offset_table: OffsetTable,
    pub char_mapping: CharMapping,
    pub font_header: FontHeader,
    pub horizontal_header: HorizontalHeader,
    pub horizontal_metrics: HorizontalMetrics,
    pub maximum_profile: MaximumProfile,
    pub naming_table: NamingTable,
    pub postscript: PostScript,
    pub windows_metrics: WindowsMetrics,
}

#[cfg(target_endian = "big")]
macro_rules! tag(
    ($value:expr) => (unsafe {
        mem::transmute::<_, [u8; 4]>($value)
    });
);

#[cfg(target_endian = "little")]
macro_rules! tag(
    ($value:expr) => (unsafe {
        let mut value = mem::transmute::<_, [u8; 4]>($value);
        value.reverse();
        value
    });
);

macro_rules! verify_and_position(
    ($record:ident, $band:ident, $table:expr, $process:expr) => ({
        if !try!($record.check($band, $process)) {
            raise!(concat!("the ", $table, " is corrupted"));
        }
        try!($band.jump($record.offset as u64));
    });
    ($record:ident, $band:ident, $table:expr) => (
        verify_and_position!($record, $band, $table, |_, word| word);
    );
);

impl Font {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        macro_rules! some(
            ($option:expr, $table:expr) => (match $option {
                Some(value) => value,
                _ => raise!(concat!("the ", $table, " is missing")),
            });
        );

        macro_rules! sort(
            ($records:expr) => ({
                let mut records = $records.iter().collect::<Vec<_>>();
                records.sort_by(|one, two| {
                    priority(&tag!(one.tag)).cmp(&priority(&tag!(two.tag)))
                });
                records
            });
        );

        let offset_table = try!(read_offset_table(reader));

        let mut char_mapping = None;
        let mut font_header = None;
        let mut horizontal_header = None;
        let mut horizontal_metrics = None;
        let mut maximum_profile = None;
        let mut naming_table = None;
        let mut postscript = None;
        let mut windows_metrics = None;

        for record in sort!(offset_table.records) {
            match &tag!(record.tag) {
                b"cmap" => char_mapping = Some(try!(read_char_mapping(reader, record))),
                b"head" => font_header = Some(try!(read_font_header(reader, record))),
                b"hhea" => horizontal_header = Some(try!(read_horizontal_header(reader, record))),
                b"hmtx" => {
                    let header = some!(horizontal_header.as_ref(), "horizontal header");
                    let profile = some!(maximum_profile.as_ref(), "maximum profile");
                    horizontal_metrics = Some(try!(read_horizontal_metrics(reader, record, header,
                                                                           profile)));
                },
                b"maxp" => maximum_profile = Some(try!(read_maximum_profile(reader, record))),
                b"name" => naming_table = Some(try!(read_naming_table(reader, record))),
                b"post" => postscript = Some(try!(read_postscript(reader, record))),
                b"OS/2" => windows_metrics = Some(try!(read_windows_metrics(reader, record))),
                _ => (),
            }
        }

        Ok(Font {
            offset_table: offset_table,
            char_mapping: some!(char_mapping, "character-to-glyph mapping"),
            font_header: some!(font_header, "font header"),
            horizontal_header: some!(horizontal_header, "horizontal header"),
            horizontal_metrics: some!(horizontal_metrics, "horizontal metrics"),
            maximum_profile: some!(maximum_profile, "maximum profile"),
            naming_table: some!(naming_table, "naming table"),
            postscript: some!(postscript, "PostScript information"),
            windows_metrics: some!(windows_metrics, "OS/2 and Windows metrics"),
        })
    }
}

fn read_offset_table<T: Band>(band: &mut T) -> Result<OffsetTable> {
    let header = match &tag!(try!(band.peek::<Fixed>())) {
        b"OTTO" => try!(OffsetTableHeader::read(band)),
        _ => raise!("the format of a font is not supported"),
    };
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(OffsetTableRecord::read(band)));
    }
    Ok(OffsetTable { header: header, records: records })
}

fn read_char_mapping<T: Band>(band: &mut T, record: &OffsetTableRecord) -> Result<CharMapping> {
    verify_and_position!(record, band, "character-to-glyph mapping");
    let header = match try!(band.peek::<USHORT>()) {
        0 => try!(CharMappingHeader::read(band)),
        _ => raise!("the format of the character-to-glyph mapping header is not supported"),
    };
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(CharMappingRecord::read(band)));
    }
    let mut encodings = vec![];
    for encoding in records.iter() {
        try!(band.jump(record.offset as u64 + encoding.offset as u64));
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

    verify_and_position!(record, band, "font header", |i, word| if i == 2 { 0 } else { word });
    let table = match try!(band.peek::<Fixed>()) {
        Fixed(0x00010000) => try!(FontHeader::read(band)),
        _ => raise!("the format of the font header is not supported"),
    };
    if table.magicNumber != MAGIC_NUMBER {
        raise!("the font header is malformed");
    }
    Ok(table)
}

fn read_horizontal_header<T: Band>(band: &mut T, record: &OffsetTableRecord)
                                   -> Result<HorizontalHeader> {

    verify_and_position!(record, band, "horizontal header");
    Ok(match try!(band.peek::<Fixed>()) {
        Fixed(0x00010000) => try!(HorizontalHeader::read(band)),
        _ => raise!("the format of the horizontal header is not supported"),
    })
}

fn read_horizontal_metrics<T: Band>(band: &mut T, record: &OffsetTableRecord,
                                    header: &HorizontalHeader, profile: &MaximumProfile)
                                    -> Result<HorizontalMetrics> {

    verify_and_position!(record, band, "horizontal metrics");
    Ok(try!(HorizontalMetrics::read(band, header, profile)))
}

fn read_maximum_profile<T: Band>(band: &mut T, record: &OffsetTableRecord)
                                 -> Result<MaximumProfile> {

    verify_and_position!(record, band, "maximum profile");
    Ok(match try!(band.peek::<Fixed>()) {
        Fixed(0x00005000) => MaximumProfile::Version05(try!(Value::read(band))),
        Fixed(0x00010000) => MaximumProfile::Version10(try!(Value::read(band))),
        _ => raise!("the format of the maximum profile is not supported"),
    })
}

fn read_naming_table<T: Band>(band: &mut T, record: &OffsetTableRecord) -> Result<NamingTable> {
    verify_and_position!(record, band, "naming table");
    Ok(match try!(band.peek::<USHORT>()) {
        0 => NamingTable::Format0(try!(Value::read(band))),
        1 => NamingTable::Format1(try!(Value::read(band))),
        _ => raise!("the format of the naming table is not supported"),
    })
}

fn read_postscript<T: Band>(band: &mut T, record: &OffsetTableRecord) -> Result<PostScript> {
    verify_and_position!(record, band, "PostScript information");
    Ok(match try!(band.peek::<Fixed>()) {
        Fixed(0x00010000) => PostScript::Version10(try!(Value::read(band))),
        Fixed(0x00030000) => PostScript::Version30(try!(Value::read(band))),
        _ => raise!("the format of the PostScript information is not supported"),
    })
}

fn read_windows_metrics<T: Band>(band: &mut T, record: &OffsetTableRecord)
                                 -> Result<WindowsMetrics> {

    verify_and_position!(record, band, "OS/2 and Windows metrics");
    Ok(match try!(band.peek::<USHORT>()) {
        3 => WindowsMetrics::Version3(try!(Value::read(band))),
        5 => WindowsMetrics::Version5(try!(Value::read(band))),
        _ => raise!("the format of the OS/2 and Windows metrics is not supported"),
    })
}

fn priority(tag: &[u8; 4]) -> usize {
    use std::collections::HashMap;
    use std::sync::{Once, ONCE_INIT};

    unsafe {
        static mut PRIORITY: *const HashMap<[u8; 4], usize> = 0 as *const _;
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut map: HashMap<[u8; 4], usize> = HashMap::new();
            map.insert(*b"hmtx", 42);
            PRIORITY = mem::transmute(Box::new(map));
        });
        *(&*PRIORITY).get(tag).unwrap_or(&0)
    }
}
