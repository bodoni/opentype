//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use {Result, Tape, Value, q32};

pub mod feature;
pub mod lookup;
pub mod script;

use self::feature::Features;
use self::lookup::Lookups;
use self::script::Scripts;

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
