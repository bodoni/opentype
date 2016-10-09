//! The [glyph-definition table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GDEF.htm

use truetype::{Result, Tape, Value};

use layout::Class;
use variation::item::Variations;

mod element;

pub use self::element::*;

macro_rules! field(
    ($table:expr => $field:ident, $enumeration:ident::{$($variant:ident),*}) => (
        match $table {
            $($enumeration::$variant(ref table) => table.$field,)*
        }
    );
    ($table:expr => $field:ident($default:expr), $enumeration:ident::{$($variant:ident),*}) => (
        match $table {
            $($enumeration::$variant(ref table) => table.$field,)*
            _ => $default,
        }
    );
);

table! {
    @position
    #[doc = "A glyph-definition table."]
    pub GlyphDefinition {
        header (Header),

        glyph_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => glyph_class_offset,
                                                    Header::{Version1, Version12, Version13}))
        },

        attachments (Option<Attachments>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => attachments_offset,
                                                    Header::{Version1, Version12, Version13}))
        },

        ligatures (Option<Ligatures>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => ligatures_offset,
                                                    Header::{Version1, Version12, Version13}))
        },

        mark_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => mark_class_offset,
                                                    Header::{Version1, Version12, Version13}))
        },

        marks (Option<Marks>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => marks_offset(0),
                                                    Header::{Version12, Version13}))
        },

        variations (Option<Variations>) |this, tape, position| {
            jump_take_maybe!(tape, position, field!(this.header => marks_offset(0),
                                                    Header::{Version13}))
        },
    }
}

/// The header of a glyph-definition table.
#[derive(Clone, Debug)]
pub enum Header {
    /// Version 1.0.
    Version1(Header1),
    /// Version 1.2.
    Version12(Header12),
    /// Version 1.3.
    Version13(Header13),
}

table! {
    #[doc = "The header of a glyph-definition table of version 1.0."]
    #[derive(Copy)]
    pub Header1 {
        major_version      (u16) = { 1 }, // MajorVersion
        minor_version      (u16) = { 0 }, // MinorVersion
        glyph_class_offset (u16), // GlyphClassDef
        attachments_offset (u16), // AttachList
        ligatures_offset   (u16), // LigCaretList
        mark_class_offset  (u16), // MarkAttachClassDef
    }
}

table! {
    #[doc = "The header of a glyph-definition table of version 1.2."]
    #[derive(Copy)]
    pub Header12 {
        major_version      (u16) = { 1 }, // MajorVersion
        minor_version      (u16) = { 2 }, // MinorVersion
        glyph_class_offset (u16), // GlyphClassDef
        attachments_offset (u16), // AttachList
        ligatures_offset   (u16), // LigCaretList
        mark_class_offset  (u16), // MarkAttachClassDef
        marks_offset       (u16), // MarkGlyphSetsDef
    }
}

table! {
    #[doc = "The header of a glyph-definition table of version 1.3."]
    #[derive(Copy)]
    pub Header13 {
        major_version      (u16) = { 1 }, // MajorVersion
        minor_version      (u16) = { 3 }, // MinorVersion
        glyph_class_offset (u16), // GlyphClassDef
        attachments_offset (u16), // AttachList
        ligatures_offset   (u16), // LigCaretList
        mark_class_offset  (u16), // MarkAttachClassDef
        marks_offset       (u16), // MarkGlyphSetsDef
        variations_offset  (u32), // ItemVarStore
    }
}

impl Value for Header {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u32>()) {
            0x00010000 => Header::Version1(try!(tape.take())),
            0x00010002 => Header::Version12(try!(tape.take())),
            0x00010003 => Header::Version13(try!(tape.take())),
            _ => raise!("found an unknown format of the glyph-definition table"),
        })
    }
}
