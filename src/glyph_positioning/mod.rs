//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use truetype::Tag;

use {Result, Tape, Value, Walue, q32};

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
            let value = read_value!(tape);
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

table! {
    #[doc = "A script list."]
    pub Scripts {
        count (u16), // ScriptCount

        records (Vec<Script>) |tape, this| { // ScriptRecord
            Walue::read(tape, this.count as usize)
        },
    }
}

table! {
    #[doc = "A feature list."]
    pub Features {
        count (u16),
    }
}

table! {
    #[doc = "A lookup list."]
    pub Lookups {
        count (u16),
    }
}

table! {
    #[doc = "A record of a script list."]
    #[derive(Copy)]
    pub Script {
        tag    (Tag), // ScriptTag
        offset (u16), // Script
    }
}

impl Value for GlyphPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let header = read_value!(tape, Header);
        try!(tape.jump(position + header.script_offset as u64));
        let scripts = read_value!(tape);
        try!(tape.jump(position + header.feature_offset as u64));
        let features = read_value!(tape);
        try!(tape.jump(position + header.lookup_offset as u64));
        let lookups = read_value!(tape);
        Ok(GlyphPositioning {
            header: header,
            scripts: scripts,
            features: features,
            lookups: lookups,
        })
    }
}
