//! The [glyph-definition table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GDEF.htm

use truetype::{Result, Tape, Value};

/// A glyph-definition table.
pub struct GlyphDefinition;

/// The reader of a glyph-definition table.
pub enum Header {
    /// Version 0x00010000.
    Version1(Header1),
    /// Version 0x00010002.
    Version12(Header12),
}

table! {
    #[doc = "The reader of a glyph-definition table of version 0x00010000."]
    pub Header1 {
        version (u32) = { 0x00010000 },
    }
}

table! {
    #[doc = "The reader of a glyph-definition table of version 0x00010002."]
    pub Header12 {
        version (u32) = { 0x00010002 },
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
