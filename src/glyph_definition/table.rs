use truetype::{Result, Tape, Value};

use glyph_definition::{Attachments, Ligatures, Marks};
use glyph_transformation::Class;

table! {
    @position
    #[doc = "A glyph-definition table."]
    pub GlyphDefinition {
        header (Header),

        glyph_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(tape, position, match this.header {
                Header::Version1(ref header) => header.glyph_class_offset,
                Header::Version12(ref header) => header.glyph_class_offset,
            })
        },

        attachments (Option<Attachments>) |this, tape, position| {
            jump_take_maybe!(tape, position, match this.header {
                Header::Version1(ref header) => header.attachments_offset,
                Header::Version12(ref header) => header.attachments_offset,
            })
        },

        ligatures (Option<Ligatures>) |this, tape, position| {
            jump_take_maybe!(tape, position, match this.header {
                Header::Version1(ref header) => header.ligatures_offset,
                Header::Version12(ref header) => header.ligatures_offset,
            })
        },

        mark_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(tape, position, match this.header {
                Header::Version1(ref header) => header.mark_class_offset,
                Header::Version12(ref header) => header.mark_class_offset,
            })
        },

        marks (Option<Marks>) |this, tape, position| {
            jump_take_maybe!(tape, position, match this.header {
                Header::Version12(ref header) => header.marks_offset,
                _ => 0,
            })
        },
    }
}

/// The header of a glyph-definition table.
#[derive(Clone, Debug)]
pub enum Header {
    /// Version 0x00010000.
    Version1(Header1),
    /// Version 0x00010002.
    Version12(Header12),
}

table! {
    #[doc = "The header of a glyph-definition table of version 0x00010000."]
    #[derive(Copy)]
    pub Header1 {
        version            (u32) = { 0x00010000 }, // Version
        glyph_class_offset (u16), // GlyphClassDef
        attachments_offset (u16), // AttachList
        ligatures_offset   (u16), // LigCaretList
        mark_class_offset  (u16), // MarkAttachClassDef
    }
}

table! {
    #[doc = "The header of a glyph-definition table of version 0x00010002."]
    #[derive(Copy)]
    pub Header12 {
        version            (u32) = { 0x00010002 },
        glyph_class_offset (u16), // GlyphClassDef
        attachments_offset (u16), // AttachList
        ligatures_offset   (u16), // LigCaretList
        mark_class_offset  (u16), // MarkAttachClassDef
        marks_offset       (u16), // MarkGlyphSetsDef
    }
}

impl Value for Header {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u32>()) {
            0x00010000 => Header::Version1(try!(tape.take())),
            0x00010002 => Header::Version12(try!(tape.take())),
            _ => raise!("found an unknown format of the glyph-definition table"),
        })
    }
}
