//! The [glyph positioning][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/gpos

mod element;

pub use element::*;

use crate::layout::{ChainedContext, Class, Context, Coverage, Directory};
use crate::{Result, Tape, Value, Walue};

/// A glyph positioning.
pub type GlyphPositioning = Directory<Type>;

/// A glyph-positioning type.
#[derive(Clone, Debug)]
pub enum Type {
    SingleAdjustment(SingleAdjustment),
    PairAdjustment(PairAdjustment),
    CursiveAttachment(CursiveAttachment),
    MarkToBaseAttachment(MarkToBaseAttachment),
    MarkToLigatureAttachment(MarkToLigatureAttachment),
    MarkToMarkAttachment(MarkToMarkAttachment),
    ContextualPositioning(Context),
    ChainedContextualPositioning(ChainedContext),
    ExtensionPositioning(ExtensionPositioning),
}

/// A single adjustment.
#[derive(Clone, Debug)]
pub enum SingleAdjustment {
    /// Format 1.
    Format1(SingleAdjustment1),
    /// Format 2.
    Format2(SingleAdjustment2),
}

table! {
    @position
    #[doc = "A single adjustment in format 1."]
    pub SingleAdjustment1 { // SinglePosFormat1
        format          (u16  ), // posFormat
        coverage_offset (u16  ), // coverageOffset
        value_flags     (Flags), // valueFormat

        value (Single) |this, tape, position| { // valueRecord
            tape.take_given((position, this.value_flags))
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A single adjustment in format 2."]
    pub SingleAdjustment2 { // SinglePosFormat2
        format          (u16  ), // posFormat
        coverage_offset (u16  ), // coverageOffset
        value_flags     (Flags), // valueFormat
        value_count     (u16  ), // valueCount

        values (Vec<Single>) |this, tape, position| { // valueRecords
            (0..this.value_count)
                .map(|_| tape.take_given((position, this.value_flags)))
                .collect()
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

/// A pair adjustment.
#[derive(Clone, Debug)]
pub enum PairAdjustment {
    /// Format 1.
    Format1(PairAdjustment1),
    /// Format 2.
    Format2(PairAdjustment2),
}

table! {
    @position
    #[doc = "A pair adjustment in format 1."]
    pub PairAdjustment1 { // PairPosFormat1
        format          (u16  ), // posFormat
        coverage_offset (u16  ), // coverageOffset
        value1_flags    (Flags), // valueFormat1
        value2_flags    (Flags), // valueFormat2
        rule_count      (u16  ), // pairSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // pairSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<Pair1s>) |this, tape, position| {
            jump_take_given!(
                tape,
                position,
                this.rule_count,
                this.rule_offsets,
                (this.value1_flags, this.value2_flags)
            )
        },
    }
}

table! {
    @position
    #[doc = "A pair adjustment in format 2."]
    pub PairAdjustment2 { // PairPosFormat2
        format          (u16  ), // posFormat
        coverage_offset (u16  ), // coverageOffset
        value1_flags    (Flags), // valueFormat1
        value2_flags    (Flags), // valueFormat2
        class1_offset   (u16  ), // classDef1Offset
        class2_offset   (u16  ), // classDef2Offset
        class1_count    (u16  ), // class1Count
        class2_count    (u16  ), // class2Count

        rules (Vec<Pair2s>) |this, tape, position| { // class1Records
            (0..this.class1_count)
                .map(|_| tape.take_given((position, this.class2_count, this.value1_flags, this.value2_flags)))
                .collect()
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        class1 (Class) |this, tape, position| {
            jump_take!(tape, position, this.class1_offset)
        },

        class2 (Class) |this, tape, position| {
            jump_take!(tape, position, this.class2_offset)
        },
    }
}

table! {
    @position
    #[doc = "A cursive attachment."]
    pub CursiveAttachment { // CursivePosFormat1
        format           (u16) = { 1 }, // posFormat
        coverage_offset  (u16), // coverageOffset
        connection_count (u16), // entryExitCount

        connections (Vec<Connection>) |this, tape, position| { // entryExitRecords
            (0..this.connection_count)
                .map(|_| tape.take_given(position))
                .collect()
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A mark-to-base attachment."]
    pub MarkToBaseAttachment { // MarkBasePosFormat1
        format               (u16) = { 1 }, // posFormat
        mark_coverage_offset (u16), // markCoverageOffset
        base_coverage_offset (u16), // baseCoverageOffset
        mark_class_count     (u16), // markClassCount
        mark_offset          (u16), // markArrayOffset
        base_offset          (u16), // baseArrayOffset

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        base_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.base_coverage_offset)
        },

        marks (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark_offset)
        },

        bases (Bases) |this, tape, position| {
            jump_take_given!(tape, position, this.base_offset, this.mark_class_count)
        },
    }
}

table! {
    @position
    #[doc = "A mark-to-ligature attachment."]
    pub MarkToLigatureAttachment { // MarkLigPosFormat1
        format                   (u16) = { 1 }, // posFormat
        mark_coverage_offset     (u16), // markCoverageOffset
        ligature_coverage_offset (u16), // ligatureCoverageOffset
        mark_class_count         (u16), // markClassCount
        mark_offset              (u16), // markArrayOffset
        ligature_offset          (u16), // ligatureArrayOffset

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        ligature_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.ligature_coverage_offset)
        },

        marks (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark_offset)
        },

        ligatures (Ligatures) |this, tape, position| {
            jump_take_given!(tape, position, this.ligature_offset, this.mark_class_count)
        },
    }
}

table! {
    @position
    #[doc = "A mark-to-mark attachment."]
    pub MarkToMarkAttachment { // MarkMarkPosFormat1
        format                (u16) = { 1 }, // posFormat
        mark1_coverage_offset (u16), // mark1CoverageOffset
        mark2_coverage_offset (u16), // mark2CoverageOffset
        mark_class_count      (u16), // markClassCount
        mark1_offset          (u16), // mark1ArrayOffset
        mark2_offset          (u16), // mark2ArrayOffset

        mark1_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark1_coverage_offset)
        },

        mark2_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark2_coverage_offset)
        },

        mark1s (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark1_offset)
        },

        mark2s (Mark2s) |this, tape, position| {
            jump_take_given!(tape, position, this.mark2_offset, this.mark_class_count)
        },
    }
}

table! {
    #[doc = "An extension positioning."]
    pub ExtensionPositioning { // ExtensionPosFormat1
        format (u16) = { 1 }, // posFormat
        r#type (u16), // extensionLookupType
        offset (u32), // extensionOffset
    }
}

impl Walue<'static> for Type {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, r#type: u16) -> Result<Self> {
        Ok(match r#type {
            1 => Self::SingleAdjustment(tape.take()?),
            2 => Self::PairAdjustment(tape.take()?),
            3 => Self::CursiveAttachment(tape.take()?),
            4 => Self::MarkToBaseAttachment(tape.take()?),
            5 => Self::MarkToLigatureAttachment(tape.take()?),
            6 => Self::MarkToMarkAttachment(tape.take()?),
            7 => Self::ContextualPositioning(tape.take()?),
            8 => Self::ChainedContextualPositioning(tape.take()?),
            9 => Self::ExtensionPositioning(tape.take()?),
            value => raise!("found an unknown type of glyph positioning ({value})"),
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the single adjustment ({value})"),
        })
    }
}

impl Value for PairAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the pair adjustment ({value})"),
        })
    }
}
