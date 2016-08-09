//! The tables.

#![allow(unused_mut, unused_variables)]

use truetype::GlyphID;

use {Result, Tape, Value, Walue};
use layout::Coverage;

/// A table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Table {
    SingleSubstibution(SingleSubstibution),
    MultipleSubstibution(MultipleSubstibution),
    AlternateSubstibution(AlternateSubstibution),
    LigatureSubstibution(LigatureSubstibution),
    ContextSubstibution(ContextSubstibution),
    ChainedContextSubstibution(ChainedContextSubstibution),
    ExtensionSubstibution(ExtensionSubstibution),
    ReverseChainedContextSubstibution(ReverseChainedContextSubstibution),
}

table! {
    #[doc = "A table for substituting one glyph with one of many glyphs."]
    pub AlternateSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in a chained context."]
    pub ChainedContextSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in a context."]
    pub ContextSubstibution {
    }
}

table! {
    #[doc = "A table for other types of substitution."]
    pub ExtensionSubstibution {
    }
}

table! {
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub LigatureSubstibution {
    }
}

table! {
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub MultipleSubstibution {
    }
}

/// A table for substituting one glyph with one glyph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleSubstibution {
    /// Format 1.
    Format1(SingleSubstibution1),
    /// Format 2.
    Format2(SingleSubstibution2),
}

table! {
    @define
    #[doc = "A table for substituting one glyph with one glyph in format 1."]
    pub SingleSubstibution1 {
        format          (u16     ), // SubstFormat
        coverage_offset (u16     ), // Coverage
        delta_glyph_id  (i16     ), // DeltaGlyphID
        coverage        (Coverage),
    }
}

table! {
    @define
    #[doc = "A table for substituting one glyph with one glyph in format 2."]
    pub SingleSubstibution2 {
        format          (u16         ), // SubstFormat
        coverage_offset (u16         ), // Coverage
        glyph_count     (u16         ), // GlyphCount
        glyph_ids       (Vec<GlyphID>), // Substitute
        coverage        (Coverage    ),
    }
}

table! {
    #[doc = "A table for substituting glyphs in reverse order in a chained context."]
    pub ReverseChainedContextSubstibution {
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleSubstibution(try!(tape.take())),
            2 => Table::MultipleSubstibution(try!(tape.take())),
            3 => Table::AlternateSubstibution(try!(tape.take())),
            4 => Table::LigatureSubstibution(try!(tape.take())),
            5 => Table::ContextSubstibution(try!(tape.take())),
            6 => Table::ChainedContextSubstibution(try!(tape.take())),
            7 => Table::ExtensionSubstibution(try!(tape.take())),
            8 => Table::ReverseChainedContextSubstibution(try!(tape.take())),
            _ => raise!("found an unknown glyph-substitution type"),
        })
    }
}

impl Value for SingleSubstibution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleSubstibution::Format1(try!(tape.take())),
            2 => SingleSubstibution::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the single-substitution table"),
        })
    }
}

impl Value for SingleSubstibution1 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let delta_glyph_id = try!(tape.take());
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleSubstibution1 {
            format: format,
            coverage_offset: coverage_offset,
            delta_glyph_id: delta_glyph_id,
            coverage: coverage,
        })
    }
}

impl Value for SingleSubstibution2 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let glyph_count = try!(tape.take());
        let glyph_ids = try!(tape.take_given(glyph_count as usize));
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleSubstibution2 {
            format: format,
            coverage_offset: coverage_offset,
            glyph_count: glyph_count,
            glyph_ids: glyph_ids,
            coverage: coverage,
        })
    }
}
