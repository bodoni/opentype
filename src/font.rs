use postscript;
use postscript::compact::FontSet;
use std::io::{Read, Seek};
use truetype::{self, Result, Tag, Tape};
use truetype::{
    CharMapping,
    FontHeader,
    GlyphData,
    GlyphMapping,
    HorizontalHeader,
    HorizontalMetrics,
    MaximumProfile,
    NamingTable,
    PostScript,
    WindowsMetrics,
};
use truetype::offset_table::{OffsetTable, Record};

use {GlyphPositioning, GlyphSubstitution};

/// A font.
pub struct Font {
    /// The offset table.
    pub offset_table: OffsetTable,
}

macro_rules! find_check_jump(
    ($this:ident, $tape:ident, $tag:expr, $process:expr) => (
        match $this.find(Tag(*$tag)) {
            Some(record) => {
                if !try!(record.checksum($tape, $process)) {
                    raise!("found a malformed font table");
                }
                try!(Tape::jump($tape, record.offset as u64));
            },
            _ => return Ok(None),
        }
    );
    ($record:ident, $tape:ident, $tag:expr) => (
        find_check_jump!($record, $tape, $tag, |_, word| word);
    );
);

macro_rules! read {
    ($($tag:expr => $method:ident => $kind:ident($($dependency:ident),+),)+) => (
        $(
            pub fn $method<'l, T>(&self, tape: &mut T, dependency: ($(&'l $dependency),+))
                                  -> Result<Option<$kind>> where T: Read + Seek {

                find_check_jump!(self, tape, $tag);
                Ok(Some(try!(Tape::take_given(tape, dependency))))
            }
        )+
    );
    ($($tag:expr => $method:ident => $kind:ident,)+) => (
        $(
            pub fn $method<T>(&self, tape: &mut T) -> Result<Option<$kind>> where T: Read + Seek {
                find_check_jump!(self, tape, $tag);
                Ok(Some(try!(Tape::take(tape))))
            }
        )+
    );
}

impl Font {
    /// Read a font.
    pub fn read<T>(tape: &mut T) -> Result<Font> where T: Read + Seek {
        Ok(Font { offset_table: try!(truetype::Value::read(tape)) })
    }

    pub fn font_header<T>(&self, tape: &mut T) -> Result<Option<FontHeader>> where T: Read + Seek {
        find_check_jump!(self, tape, b"head", |i, word| if i == 2 { 0 } else { word });
        Ok(Some(try!(Tape::take(tape))))
    }

    pub fn font_set<T>(&self, tape: &mut T) -> Result<Option<FontSet>> where T: Read + Seek {
        find_check_jump!(self, tape, b"CFF ");
        Ok(Some(try!(postscript::Tape::take(tape))))
    }

    read! {
        b"GPOS" => glyph_positioning => GlyphPositioning,
        b"GSUB" => glyph_substitution => GlyphSubstitution,
        b"OS/2" => windows_metrics => WindowsMetrics,
        b"cmap" => char_mapping => CharMapping,
        b"hhea" => horizontal_header => HorizontalHeader,
        b"maxp" => maximum_profile => MaximumProfile,
        b"name" => naming_table => NamingTable,
        b"post" => postscript => PostScript,
    }

    read! {
        b"glyf" => glyph_data => GlyphData(GlyphMapping),
        b"hmtx" => horizontal_metrics => HorizontalMetrics(HorizontalHeader, MaximumProfile),
        b"loca" => glyph_mapping => GlyphMapping(FontHeader, MaximumProfile),
    }

    fn find(&self, tag: Tag) -> Option<&Record> {
        for record in &self.offset_table.records {
            if record.tag == tag {
                return Some(record);
            }
        }
        None
    }
}
