//! The [glyph positioning][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/gpos

mod element;

pub use element::*;

use crate::layout::{ChainedContext, Class, Context, Coverage, Directory};
use crate::{Result, Tape, Value, Walue};

/// A glyph-positioning table.
pub type GlyphPositioning = Directory<Table>;

/// An inner table of a glyph-positioning table.
#[derive(Clone, Debug)]
pub enum Table {
    SingleAdjustment(SingleAdjustment),
    PairAdjustment(PairAdjustment),
    CursiveAttachment(CursiveAttachment),
    MarkToBaseAttachment(MarkToBaseAttachment),
    MarkToLigatureAttachment(MarkToLigatureAttachment),
    MarkToMarkAttachment(MarkToMarkAttachment),
    Context(Context),
    ChainedContext(ChainedContext),
    Extension(Extension),
}

/// A table for adjusting single glyphs.
#[derive(Clone, Debug)]
pub enum SingleAdjustment {
    /// Format 1.
    Format1(SingleAdjustment1),
    /// Format 2.
    Format2(SingleAdjustment2),
}

table! {
    @position
    #[doc = "A table for adjusting single glyphs in format 1."]
    pub SingleAdjustment1 { // SinglePosFormat1
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value_flags     (SingleFlags), // valueFormat

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
    #[doc = "A table for adjusting single glyphs in format 2."]
    pub SingleAdjustment2 { // SinglePosFormat2
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value_flags     (SingleFlags), // valueFormat
        value_count     (u16        ), // valueCount

        values (Vec<Single>) |this, tape, position| { // valueRecords
            let mut values = Vec::with_capacity(this.value_count as usize);
            for _ in 0..(this.value_count as usize) {
                values.push(tape.take_given((position, this.value_flags))?);
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

/// A table for adjusting pairs of glyphs.
#[derive(Clone, Debug)]
pub enum PairAdjustment {
    /// Format 1.
    Format1(PairAdjustment1),
    /// Format 2.
    Format2(PairAdjustment2),
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 1."]
    pub PairAdjustment1 { // PairPosFormat1
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value1_flags    (SingleFlags), // valueFormat1
        value2_flags    (SingleFlags), // valueFormat2
        rule_count      (u16        ), // pairSetCount

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
                (position, this.value1_flags, this.value2_flags)
            )
        },
    }
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 { // PairPosFormat2
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value1_flags    (SingleFlags), // valueFormat1
        value2_flags    (SingleFlags), // valueFormat2
        class1_offset   (u16        ), // classDef1Offset
        class2_offset   (u16        ), // classDef2Offset
        class1_count    (u16        ), // class1Count
        class2_count    (u16        ), // class2Count

        rules (Vec<Pair2s>) |this, tape, position| { // class1Records
            let mut values = Vec::with_capacity(this.class1_count as usize);
            for _ in 0..(this.class1_count as usize) {
                values.push(tape.take_given((
                    position,
                    this.class2_count,
                    this.value1_flags,
                    this.value2_flags,
                ))?);
            }
            Ok(values)
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
    #[doc = "A table for attaching cursive glyphs."]
    pub CursiveAttachment { // CursivePosFormat1
        format          (u16) = { 1 }, // posFormat
        coverage_offset (u16), // coverageOffset
        passage_count   (u16), // entryExitCount

        passages (Vec<Passage>) |this, tape, position| { // entryExitRecords
            let mut values = Vec::with_capacity(this.passage_count as usize);
            for _ in 0..(this.passage_count as usize) {
                values.push(tape.take_given(position)?);
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to base glyphs."]
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
    #[doc = "A table for attaching combining marks to ligatures."]
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
    #[doc = "A table for attaching combining marks to other marks."]
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
    #[doc = "A table for other types of positioning."]
    pub Extension { // ExtensionPosFormat1
        format (u16) = { 1 }, // posFormat
        kind   (u16), // extensionLookupType
        offset (u32), // extensionOffset
    }
}

impl Walue<'static> for Table {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Self::SingleAdjustment(tape.take()?),
            2 => Self::PairAdjustment(tape.take()?),
            3 => Self::CursiveAttachment(tape.take()?),
            4 => Self::MarkToBaseAttachment(tape.take()?),
            5 => Self::MarkToLigatureAttachment(tape.take()?),
            6 => Self::MarkToMarkAttachment(tape.take()?),
            7 => Self::Context(tape.take()?),
            8 => Self::ChainedContext(tape.take()?),
            9 => Self::Extension(tape.take()?),
            value => raise!("found an unknown glyph-positioning type ({value})"),
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the single-adjustment table ({value})"),
        })
    }
}

impl Value for PairAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the pair-adjustment table ({value})"),
        })
    }
}
