use std::io::{Read, Seek};
use std::mem;

use postscript;
use postscript::compact::FontSet;
use truetype::{self, Tag, q32};
use truetype::{
    CharMapping,
    FontHeader,
    GlyphData,
    GlyphMapping,
    HorizontalHeader,
    HorizontalMetrics,
    MaximumProfile,
    NamingTable,
    OffsetTable,
    PostScript,
    WindowsMetrics,
};

use Result;

/// A font.
pub struct Font {
    /// The offset table.
    pub offset_table: OffsetTable,

    /// The char-to-glyph mapping (`cmap`).
    pub char_mapping: Option<CharMapping>,
    /// The compact font set (`CFF `).
    pub compact_font_set: Option<FontSet>,
    /// The font header (`head`).
    pub font_header: Option<FontHeader>,
    /// The glyph data (`glyf`).
    pub glyph_data: Option<GlyphData>,
    /// The glyph-to-location mapping (`loca`).
    pub glyph_mapping: Option<GlyphMapping>,
    /// The horizontal header (`hhea`).
    pub horizontal_header: Option<HorizontalHeader>,
    /// The horizontal metrics (`hmtx`).
    pub horizontal_metrics: Option<HorizontalMetrics>,
    /// The maximum profile (`maxp`).
    pub maximum_profile: Option<MaximumProfile>,
    /// The naming table (`name`).
    pub naming_table: Option<NamingTable>,
    /// The PostScript information (`post`).
    pub postscript: Option<PostScript>,
    /// The OS/2 and Windows information (`OS/2`).
    pub windows_metrics: Option<WindowsMetrics>,
}

macro_rules! check_jump(
    ($record:ident, $tape:ident, $process:expr) => ({
        if !try!($record.checksum($tape, $process)) {
            raise!("found a malformed font table");
        }
        try!(truetype::Tape::jump($tape, $record.offset as u64));
    });
    ($record:ident, $tape:ident) => (
        check_jump!($record, $tape, |_, word| word);
    );
);

impl Font {
    /// Read a font.
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<Font> {
        macro_rules! sort(
            ($records:expr) => ({
                let mut records = $records.iter().collect::<Vec<_>>();
                records.sort_by(|one, another| priority(one.tag).cmp(&priority(another.tag)));
                records
            });
        );

        match try!(truetype::Tape::peek::<q32>(tape)) {
            q32(0x00010000) => {},
            version => match &*Tag::from(version) {
                b"OTTO" => {},
                b"ttcf" => raise!("TrueType collections are not supported yet"),
                _ => raise!("found an unknown font format"),
            },
        }

        let mut font = Font {
            offset_table: try!(truetype::Value::read(tape)),

            char_mapping: None,
            compact_font_set: None,
            font_header: None,
            glyph_data: None,
            glyph_mapping: None,
            horizontal_header: None,
            horizontal_metrics: None,
            maximum_profile: None,
            naming_table: None,
            postscript: None,
            windows_metrics: None,
        };
        for record in sort!(font.offset_table.records) {
            macro_rules! set(
                ($field:ident, $value:expr) => ({
                    check_jump!(record, tape);
                    font.$field = Some(try!($value));
                });
                ($field:ident) => (set!($field, truetype::Value::read(tape)));
            );
            macro_rules! get(
                ($field:ident) => ({
                    match font.$field {
                        Some(ref table) => table,
                        _ => continue,
                    }
                });
            );
            match &*record.tag {
                b"CFF " => set!(compact_font_set, postscript::Value::read(tape)),
                b"OS/2" => set!(windows_metrics),
                b"cmap" => set!(char_mapping),
                b"glyf" => {
                    let mapping = get!(glyph_mapping);
                    set!(glyph_data, truetype::Walue::read(tape, mapping));
                },
                b"head" => {
                    check_jump!(record, tape, |i, word| if i == 2 { 0 } else { word });
                    font.font_header = Some(try!(truetype::Value::read(tape)));
                },
                b"hhea" => set!(horizontal_header),
                b"hmtx" => {
                    let header = get!(horizontal_header);
                    let profile = get!(maximum_profile);
                    set!(horizontal_metrics, truetype::Walue::read(tape, (header, profile)));
                },
                b"loca" => {
                    let header = get!(font_header);
                    let profile = get!(maximum_profile);
                    set!(glyph_mapping, truetype::Walue::read(tape, (header, profile)));
                },
                b"maxp" => set!(maximum_profile),
                b"name" => set!(naming_table),
                b"post" => set!(postscript),
                _ => {},
            }
        }

        Ok(font)
    }
}

fn priority(tag: Tag) -> usize {
    use std::collections::HashMap;
    use std::sync::{Once, ONCE_INIT};

    unsafe {
        static mut PRIORITY: *const HashMap<Tag, usize> = 0 as *const _;
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut map: HashMap<Tag, usize> = HashMap::new();
            map.insert(Tag(*b"glyf"), 43);
            map.insert(Tag(*b"hmtx"), 42);
            map.insert(Tag(*b"loca"), 41);
            PRIORITY = mem::transmute(Box::new(map));
        });
        *(&*PRIORITY).get(&tag).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use File;

    const CFF: &'static str = "tests/fixtures/SourceSerifPro-Regular.otf";
    const TTF: &'static str = "tests/fixtures/OpenSans-Italic.ttf";

    #[test]
    fn cff() {
        let file = File::open(CFF).unwrap();
        assert!(file[0].compact_font_set.is_some());
    }

    #[test]
    fn ttf() {
        let file = File::open(TTF).unwrap();
        assert!(file[0].glyph_data.is_some());
    }
}
