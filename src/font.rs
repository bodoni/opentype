use std::io::{Read, Seek};
use std::mem;

use Result;
use truetype::{Tape, Value};
use truetype::compound::*;

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

impl Font {
    #[inline]
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<Font> {
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

macro_rules! checksum_and_jump(
    ($record:ident, $tape:ident, $table:expr, $process:expr) => ({
        if !try!($record.checksum($tape, $process)) {
            raise!(concat!("the ", $table, " is corrupted"));
        }
        try!($tape.jump($record.offset as u64));
    });
    ($record:ident, $tape:ident, $table:expr) => (
        checksum_and_jump!($record, $tape, $table, |_, word| word);
    );
);

fn read_offset_table<T: Tape>(tape: &mut T) -> Result<OffsetTable> {
    let table = try!(OffsetTable::read(tape));
    if &tag!(table.header.version) != b"OTTO" {
        raise!("the font format is invalid");
    }
    Ok(table)
}

fn read_char_mapping<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<CharMapping> {
    checksum_and_jump!(record, tape, "character-to-glyph mapping");
    CharMapping::read(tape)
}

fn read_font_header<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<FontHeader> {
    checksum_and_jump!(record, tape, "font header", |i, word| if i == 2 { 0 } else { word });
    FontHeader::read(tape)
}

fn read_horizontal_header<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                   -> Result<HorizontalHeader> {

    checksum_and_jump!(record, tape, "horizontal header");
    HorizontalHeader::read(tape)
}

fn read_horizontal_metrics<T: Tape>(tape: &mut T, record: &OffsetTableRecord,
                                    header: &HorizontalHeader, profile: &MaximumProfile)
                                    -> Result<HorizontalMetrics> {

    checksum_and_jump!(record, tape, "horizontal metrics");
    HorizontalMetrics::read(tape, header, profile)
}

fn read_maximum_profile<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                 -> Result<MaximumProfile> {

    checksum_and_jump!(record, tape, "maximum profile");
    MaximumProfile::read(tape)
}

fn read_naming_table<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<NamingTable> {
    checksum_and_jump!(record, tape, "naming table");
    NamingTable::read(tape)
}

fn read_postscript<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<PostScript> {
    checksum_and_jump!(record, tape, "PostScript information");
    PostScript::read(tape)
}

fn read_windows_metrics<T: Tape>(tape: &mut T, record: &OffsetTableRecord)
                                 -> Result<WindowsMetrics> {

    checksum_and_jump!(record, tape, "OS/2 and Windows metrics");
    WindowsMetrics::read(tape)
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
