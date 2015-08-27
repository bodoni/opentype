use std::io::{Read, Seek};
use std::mem;

use Result;
use tape::{Tape, Value};
use primitive::*;
use table::*;

/// A font.
#[derive(Default)]
pub struct Font {
    pub offset_table: OffsetTable,
    pub char_mapping: Option<CharMapping>,
    pub font_header: Option<FontHeader>,
    pub horizontal_header: Option<HorizontalHeader>,
    pub horizontal_metrics: Option<HorizontalMetrics>,
    pub maximum_profile: Option<MaximumProfile>,
    pub naming_table: Option<NamingTable>,
    pub postscript: Option<PostScript>,
    pub windows_metrics: Option<WindowsMetrics>,
}

macro_rules! tag(
    ($value:expr) => (unsafe {
        mem::transmute::<_, [u8; 4]>(u32::from_be(mem::transmute($value)))
    });
);

macro_rules! verify_and_jump(
    ($record:ident, $tape:ident, $table:expr, $process:expr) => ({
        if !try!($record.check($tape, $process)) {
            raise!(concat!("the ", $table, " is corrupted"));
        }
        try!($tape.jump($record.offset as u64));
    });
    ($record:ident, $tape:ident, $table:expr) => (
        verify_and_jump!($record, $tape, $table, |_, word| word);
    );
);

impl Font {
    #[inline]
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        Value::read(reader)
    }
}

impl Value for Font {
    fn read<T: Tape>(tape: &mut T) -> Result<Font> {
        macro_rules! sort(
            ($records:expr) => ({
                let mut records = $records.iter().collect::<Vec<_>>();
                records.sort_by(|one, two| {
                    priority(&tag!(one.tag)).cmp(&priority(&tag!(two.tag)))
                });
                records
            });
        );

        let mut font = Font { offset_table: try!(read_offset_table(tape)), .. Font::default() };

        for record in sort!(font.offset_table.records) {
            macro_rules! set(
                ($name:ident, $read:ident $($argument:tt)*) => (
                    font.$name = Some(try!($read(tape, record $($argument)*)))
                );
            );
            match &tag!(record.tag) {
                b"cmap" => set!(char_mapping, read_char_mapping),
                b"head" => set!(font_header, read_font_header),
                b"hhea" => set!(horizontal_header, read_horizontal_header),
                b"hmtx" => {
                    let header = match font.horizontal_header {
                        Some(ref table) => table,
                        _ => continue,
                    };
                    let profile = match font.maximum_profile {
                        Some(ref table) => table,
                        _ => continue,
                    };
                    set!(horizontal_metrics, read_horizontal_metrics, header, profile);
                },
                b"maxp" => set!(maximum_profile, read_maximum_profile),
                b"name" => set!(naming_table, read_naming_table),
                b"post" => set!(postscript, read_postscript),
                b"OS/2" => set!(windows_metrics, read_windows_metrics),
                _ => {},
            }
        }

        Ok(font)
    }
}

fn read_offset_table<T: Tape>(tape: &mut T) -> Result<OffsetTable> {
    let header = match &tag!(try!(tape.peek::<Fixed>())) {
        b"OTTO" => try!(OffsetTableHeader::read(tape)),
        _ => raise!("the format of a font is not supported"),
    };
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(OffsetTableRecord::read(tape)));
    }
    Ok(OffsetTable { header: header, records: records })
}

fn read_char_mapping<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<CharMapping> {
    verify_and_jump!(record, tape, "character-to-glyph mapping");
    let header = match try!(tape.peek::<UShort>()) {
        0 => try!(CharMappingHeader::read(tape)),
        _ => raise!("the format of the character-to-glyph mapping header is not supported"),
    };
    let mut records = vec![];
    for _ in 0..header.numTables {
        records.push(try!(CharMappingRecord::read(tape)));
    }
    let mut encodings = vec![];
    for encoding in records.iter() {
        try!(tape.jump(record.offset as u64 + encoding.offset as u64));
        encodings.push(match try!(tape.peek::<UShort>()) {
            4 => CharMappingEncoding::Format4(try!(Value::read(tape))),
            6 => CharMappingEncoding::Format6(try!(Value::read(tape))),
            _ => raise!("the format of a character-to-glyph mapping is not supported"),
        });
    }

    Ok(CharMapping { header: header, records: records, encodings: encodings })
}

fn read_font_header<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<FontHeader> {
    const MAGIC_NUMBER: ULong = 0x5F0F3CF5;

    verify_and_jump!(record, tape, "font header", |i, word| if i == 2 { 0 } else { word });
    let table = match try!(tape.peek::<Fixed>()) {
        Fixed(0x00010000) => try!(FontHeader::read(tape)),
        _ => raise!("the format of the font header is not supported"),
    };
    if table.magicNumber != MAGIC_NUMBER {
        raise!("the font header is malformed");
    }
    Ok(table)
}

fn read_horizontal_header<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                   -> Result<HorizontalHeader> {

    verify_and_jump!(record, tape, "horizontal header");
    Ok(match try!(tape.peek::<Fixed>()) {
        Fixed(0x00010000) => try!(HorizontalHeader::read(tape)),
        _ => raise!("the format of the horizontal header is not supported"),
    })
}

fn read_horizontal_metrics<T: Tape>(tape: &mut T, record: &OffsetTableRecord,
                                    header: &HorizontalHeader, profile: &MaximumProfile)
                                    -> Result<HorizontalMetrics> {

    verify_and_jump!(record, tape, "horizontal metrics");
    Ok(try!(HorizontalMetrics::read(tape, header, profile)))
}

fn read_maximum_profile<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                 -> Result<MaximumProfile> {

    verify_and_jump!(record, tape, "maximum profile");
    Ok(match try!(tape.peek::<Fixed>()) {
        Fixed(0x00005000) => MaximumProfile::Version05(try!(Value::read(tape))),
        Fixed(0x00010000) => MaximumProfile::Version10(try!(Value::read(tape))),
        _ => raise!("the format of the maximum profile is not supported"),
    })
}

fn read_naming_table<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<NamingTable> {
    verify_and_jump!(record, tape, "naming table");
    Ok(match try!(tape.peek::<UShort>()) {
        0 => NamingTable::Format0(try!(Value::read(tape))),
        1 => NamingTable::Format1(try!(Value::read(tape))),
        _ => raise!("the format of the naming table is not supported"),
    })
}

fn read_postscript<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<PostScript> {
    verify_and_jump!(record, tape, "PostScript information");
    Ok(match try!(tape.peek::<Fixed>()) {
        Fixed(0x00010000) => PostScript::Version10(try!(Value::read(tape))),
        Fixed(0x00030000) => PostScript::Version30(try!(Value::read(tape))),
        _ => raise!("the format of the PostScript information is not supported"),
    })
}

fn read_windows_metrics<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                 -> Result<WindowsMetrics> {

    verify_and_jump!(record, tape, "OS/2 and Windows metrics");
    Ok(match try!(tape.peek::<UShort>()) {
        3 => WindowsMetrics::Version3(try!(Value::read(tape))),
        5 => WindowsMetrics::Version5(try!(Value::read(tape))),
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
