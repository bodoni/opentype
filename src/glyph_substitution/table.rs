#![allow(unused_mut, unused_variables)]

use truetype::GlyphID;

use {Result, Tape, Value, Walue};
use glyph_substitution::{
    AlternateSet,
    ChainRuleSet,
    ClassRuleSet,
    LigatureSet,
    RuleSet,
    Sequence,
    Substitution,
};
use layout::Coverage;

/// A table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Table {
    SingleSubstitution(SingleSubstitution),
    MultipleSubstitution(MultipleSubstitution),
    AlternateSubstitution(AlternateSubstitution),
    LigatureSubstitution(LigatureSubstitution),
    ContextSubstitution(ContextSubstitution),
    ChainContextSubstitution(ChainContextSubstitution),
    ExtensionSubstitution(ExtensionSubstitution),
    ReverseChainContextSubstitution(ReverseChainContextSubstitution),
}

/// A table for substituting one glyph with one glyph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleSubstitution {
    /// Format 1.
    Format1(SingleSubstitution1),
    /// Format 2.
    Format2(SingleSubstitution2),
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 1."]
    pub SingleSubstitution1 {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        delta_glyph_id  (i16), // DeltaGlyphID

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 2."]
    pub SingleSubstitution2 {
        format          (u16) = { 2 }, // SubstFormat
        coverage_offset (u16), // Coverage
        glyph_count     (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape, _| { // Substitute
            tape.take_given(this.glyph_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub MultipleSubstitution {
        format           (u16) = { 1 }, // SubstFormat
        coverage_offset  (u16), // Coverage
        sequence_count   (u16), // SequenceCount

        sequence_offsets (Vec<u16>) |this, tape, _| { // Sequence
            tape.take_given(this.sequence_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sequences (Vec<Sequence>) |this, tape, position| {
            jump_take!(tape, position, this.sequence_count, this.sequence_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one of many glyphs."]
    pub AlternateSubstitution {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // AlternateSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // AlternateSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<AlternateSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub LigatureSubstitution {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // LigSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // LigatureSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<LigatureSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

/// A table for substituting glyphs in a context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextSubstitution {
    /// Format 1.
    Format1(ContextSubstitution1),
    /// Format 2.
    Format2(ContextSubstitution2),
    /// Format 3.
    Format3(ContextSubstitution3),
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 1."]
    pub ContextSubstitution1 {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // SubRuleSetCount

        set_offsets (Vec<u16>) |this, tape, position| { // SubRuleSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<RuleSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 2."]
    pub ContextSubstitution2 {
        format          (u16) = { 2 }, // SubstFormat
        coverage_offset (u16), // Coverage
        class_offset    (u16), // ClassDef
        set_count       (u16), // SubClassSetCnt

        set_offsets (Vec<u16>) |this, tape, _| { // SubClassSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<ClassRuleSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 3."]
    pub ContextSubstitution3 {
        format             (u16) = { 3 }, // SubstFormat
        glyph_count        (u16), // GlyphCount
        substitution_count (u16), // SubstCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.glyph_count as usize)
        },

        substitutions (Vec<Substitution>) |this, tape, _| { // SubstLookupRecord
            tape.take_given(this.substitution_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

/// A table for substituting glyphs in a chaining context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChainContextSubstitution {
    /// Format 1.
    Format1(ChainContextSubstitution1),
    /// Format 2.
    Format2(ChainContextSubstitution2),
    /// Format 3.
    Format3(ChainContextSubstitution3),
}

table! {
    #[doc = "A table for other types of substitution."]
    pub ExtensionSubstitution {
        format (u16) = { 1 }, // SubstFormat
        kind   (u16), // ExtensionLookupType
        offset (u32), // ExtensionOffset
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a chaining context in format 1."]
    pub ChainContextSubstitution1 {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // ChainSubRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // ChainSubRuleSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<ChainRuleSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    #[doc = "A table for substituting glyphs in a chaining context in format 2."]
    pub ChainContextSubstitution2 {
        format (u16) = { 2 }, // SubstFormat
    }
}

table! {
    #[doc = "A table for substituting glyphs in a chaining context in format 3."]
    pub ChainContextSubstitution3 {
        format (u16) = { 3 }, // SubstFormat
    }
}

table! {
    #[doc = "A table for substituting glyphs in reverse order in a chaining context."]
    pub ReverseChainContextSubstitution {
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleSubstitution(try!(tape.take())),
            2 => Table::MultipleSubstitution(try!(tape.take())),
            3 => Table::AlternateSubstitution(try!(tape.take())),
            4 => Table::LigatureSubstitution(try!(tape.take())),
            5 => Table::ContextSubstitution(try!(tape.take())),
            6 => Table::ChainContextSubstitution(try!(tape.take())),
            7 => Table::ExtensionSubstitution(try!(tape.take())),
            8 => Table::ReverseChainContextSubstitution(try!(tape.take())),
            _ => raise!("found an unknown glyph-substitution type"),
        })
    }
}

impl Value for SingleSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleSubstitution::Format1(try!(tape.take())),
            2 => SingleSubstitution::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the single-substitution table"),
        })
    }
}

impl Value for ContextSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => ContextSubstitution::Format1(try!(tape.take())),
            2 => ContextSubstitution::Format2(try!(tape.take())),
            3 => ContextSubstitution::Format3(try!(tape.take())),
            _ => raise!("found an unknown format of the context-substitution table"),
        })
    }
}

impl Value for ChainContextSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => ChainContextSubstitution::Format1(try!(tape.take())),
            2 => ChainContextSubstitution::Format2(try!(tape.take())),
            3 => ChainContextSubstitution::Format3(try!(tape.take())),
            _ => raise!("found an unknown format of the chaining-context-substitution table"),
        })
    }
}
