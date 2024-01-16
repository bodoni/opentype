//! The [glyph definition][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/gdef

mod element;

pub use element::*;

use crate::layout::Class;
use crate::variations::item::Store;
use crate::{Result, Tape, Value};

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
    #[doc = "A glyph definition."]
    pub GlyphDefinition {
        header (Header),

        glyph_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => glyph_class_offset, Header::{Version1, Version12, Version13})
            )
        },

        attachments (Option<Attachments>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => attachment_offset, Header::{Version1, Version12, Version13})
            )
        },

        ligatures (Option<Ligatures>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => ligature_offset, Header::{Version1, Version12, Version13})
            )
        },

        mark_class (Option<Class>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => mark_class_offset, Header::{Version1, Version12, Version13})
            )
        },

        marks (Option<Marks>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => mark_offset(0), Header::{Version12, Version13})
            )
        },

        variations (Option<Store>) |this, tape, position| {
            jump_take_maybe!(
                tape,
                position,
                field!(this.header => mark_offset(0), Header::{Version13})
            )
        },
    }
}

/// The header of a glyph definition.
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
    #[doc = "The header of a glyph definition of version 1.0."]
    #[derive(Copy)]
    pub Header1 {
        major_version      (u16) = { 1 }, // majorVersion
        minor_version      (u16) = { 0 }, // minorVersion
        glyph_class_offset (u16), // glyphClassDefOffset
        attachment_offset  (u16), // attachListOffset
        ligature_offset    (u16), // ligCaretListOffset
        mark_class_offset  (u16), // markAttachClassDefOffset
    }
}

table! {
    #[doc = "The header of a glyph definition of version 1.2."]
    #[derive(Copy)]
    pub Header12 {
        major_version      (u16) = { 1 }, // majorVersion
        minor_version      (u16) = { 2 }, // minorVersion
        glyph_class_offset (u16), // glyphClassDefOffset
        attachment_offset  (u16), // attachListOffset
        ligature_offset    (u16), // ligCaretListOffset
        mark_class_offset  (u16), // markAttachClassDefOffset
        mark_offset        (u16), // markGlyphSetsDefOffset
    }
}

table! {
    #[doc = "The header of a glyph definition of version 1.3."]
    #[derive(Copy)]
    pub Header13 {
        major_version      (u16) = { 1 }, // majorVersion
        minor_version      (u16) = { 3 }, // minorVersion
        glyph_class_offset (u16), // glyphClassDefOffset
        attachment_offset  (u16), // attachListOffset
        ligature_offset    (u16), // ligCaretListOffset
        mark_class_offset  (u16), // markAttachClassDefOffset
        mark_offset        (u16), // markGlyphSetsDefOffset
        variation_offset   (u32), // itemVarStoreOffset
    }
}

impl Default for Header {
    #[inline]
    fn default() -> Self {
        Self::Version1(Header1::default())
    }
}

impl Value for Header {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u32>()? {
            0x00010000 => Self::Version1(tape.take()?),
            0x00010002 => Self::Version12(tape.take()?),
            0x00010003 => Self::Version13(tape.take()?),
            _ => raise!("found an unknown format of the glyph definition"),
        })
    }
}
