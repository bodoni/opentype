//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use {Result, Tape, Value, q32};

use layout::{Features, Lookups, Scripts};

/// A glyph-positioning table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphPositioning {
    pub header: Header,
    pub scripts: Scripts,
    pub features: Features,
    pub lookups: Lookups,
}

table! {
    #[doc = "The header of a glyph-positioning table."]
    #[derive(Copy)]
    pub Header {
        version (q32) |tape, this| { // Version
            let value = try!(tape.take());
            if value != q32(0x00010000) {
                raise!("the version of the glyph-positioning table is not supported");
            }
            Ok(value)
        },

        script_offset  (u16), // ScriptList
        feature_offset (u16), // FeatureList
        lookup_offset  (u16), // LookupList
    }
}

impl Value for GlyphPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let header = try!(tape.take::<Header>());
        try!(tape.jump(position + header.script_offset as u64));
        let scripts = try!(tape.take());
        try!(tape.jump(position + header.feature_offset as u64));
        let features = try!(tape.take());
        try!(tape.jump(position + header.lookup_offset as u64));
        let lookups = try!(tape.take());
        Ok(GlyphPositioning {
            header: header,
            scripts: scripts,
            features: features,
            lookups: lookups,
        })
    }
}
