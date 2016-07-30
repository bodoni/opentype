//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use {Result, Tape, Value, q32};

/// A glyph-positioning table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphPositioning {
    pub header: Header,
}

table! {
    #[doc = "The header of a glyph-positioning table."]
    #[derive(Copy)]
    pub Header {
        version      (q32), // Version
        script_list  (u16), // ScriptList
        feature_list (u16), // FeatureList
        lookup_list  (u16), // LookupList
    }
}

impl Value for GlyphPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let header = read_value!(tape);
        Ok(GlyphPositioning {
            header: header,
        })
    }
}
