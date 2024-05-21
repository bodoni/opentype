//! The lookup context.

use truetype::GlyphID;

use crate::layout::{Class, Coverage};
use crate::Result;

/// A contextual lookup.
#[derive(Clone, Debug)]
pub enum Context {
    /// Format 1.
    Format1(Context1),
    /// Format 2.
    Format2(Context2),
    /// Format 3.
    Format3(Context3),
}

table! {
    @position
    /// A contextual lookup in format 1.
    pub Context1 { // SequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        record_count    (u16), // seqRuleSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // seqRuleSetOffsets
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Records>) |this, tape, position| {
            jump_take!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// A contextual lookup in format 2.
    pub Context2 { // SequenceContextFormat2
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        class_offset    (u16), // classDefOffset
        record_count    (u16), // classSeqRuleSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // classSeqRuleSetOffsets
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        class (Class) |this, tape, position| {
            jump_take!(tape, position, this.class_offset)
        },

        records (Vec<Option<ClassRecords>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// A contextual lookup in format 3.
    pub Context3 { // SequenceContextFormat3
        format       (u16), // format
        glyph_count  (u16), // glyphCount
        record_count (u16), // seqLookupCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // coverageOffsets
            tape.take_given(this.glyph_count as usize)
        },

        records (Vec<LookupRecord>) |this, tape, _| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

/// A chained contextual lookup.
#[derive(Clone, Debug)]
pub enum ChainedContext {
    /// Format 1.
    Format1(ChainedContext1),
    /// Format 2.
    Format2(ChainedContext2),
    /// Format 3.
    Format3(ChainedContext3),
}

table! {
    @position
    /// A chained contextual lookup in format 1.
    pub ChainedContext1 { // ChainedSequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        record_count    (u16), // chainedSeqRuleSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleSetOffsets
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<ChainedRecords>) |this, tape, position| {
            jump_take!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// A chained contextual lookup in format 2.
    pub ChainedContext2 { // ChainedSequenceContextFormat2
        format                (u16), // format
        coverage_offset       (u16), // coverageOffset
        backward_class_offset (u16), // backtrackClassDefOffset
        input_class_offset    (u16), // inputClassDefOffset
        forward_class_offset  (u16), // lookaheadClassDefOffset
        record_count          (u16), // chainedClassSeqRuleSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleSetOffsets
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        backward_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.backward_class_offset)
        },

        input_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.input_class_offset)
        },

        forward_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.forward_class_offset)
        },

        records (Vec<Option<ChainedClassRecords>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// A chained contextual lookup in format 3.
    pub ChainedContext3 { // ChainedSequenceContextFormat3
        format               (u16), // format
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_coverage_offsets (Vec<u16>) |this, tape, _| { // backtrackCoverageOffsets
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_coverage_offsets (Vec<u16>) |this, tape, _| { // inputCoverageOffsets
            tape.take_given(this.input_glyph_count as usize)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_coverage_offsets (Vec<u16>) |this, tape, _| { // lookaheadCoverageOffsets
            tape.take_given(this.forward_glyph_count as usize)
        },

        record_count (u16), // seqLookupCount

        records (Vec<LookupRecord>) |this, tape, _| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },

        backward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.backward_glyph_count, this.backward_coverage_offsets)
        },

        input_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.input_glyph_count, this.input_coverage_offsets)
        },

        forward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.forward_glyph_count, this.forward_coverage_offsets)
        },
    }
}

table! {
    /// A context record.
    pub Record { // SequenceRule
        glyph_count  (u16), // glyphCount
        record_count (u16), // seqLookupCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.glyph_count == 0 {
                raise!("found a malformed record");
            }
            tape.take_given(this.glyph_count as usize - 1)
        },

        records (Vec<LookupRecord>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    /// Context records.
    pub Records { // SequenceRuleSet
        count (u16), // seqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // seqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    /// A class context record.
    pub ClassRecord { // ClassSequenceRule
        glyph_count  (u16), // glyphCount
        record_count (u16), // seqLookupCount

        indices (Vec<u16>) |this, tape| { // inputSequence
            if this.glyph_count == 0 {
                raise!("found a malformed class record");
            }
            tape.take_given(this.glyph_count as usize - 1)
        },

        records (Vec<LookupRecord>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    /// Class context records.
    pub ClassRecords { // ClassSequenceRuleSet
        count (u16), // classSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // classSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ClassRecord>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    /// A chained context record.
    pub ChainedRecord { // ChainedSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_glyph_ids (Vec<GlyphID>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained record");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_glyph_ids (Vec<GlyphID>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        record_count (u16), // seqLookupCount

        records (Vec<LookupRecord>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    /// Chained context records.
    pub ChainedRecords { // ChainedSequenceRuleSet
        count (u16), // chainedSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedRecord>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    /// A chained class context record.
    pub ChainedClassRecord { // ChainedClassSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_class_ids (Vec<u16>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_class_ids (Vec<u16>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained class record");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_class_ids (Vec<u16>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        record_count (u16), // seqLookupCount

        records (Vec<LookupRecord>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    /// Chained class context records.
    pub ChainedClassRecords { // ChainedClassSequenceRuleSet
        count (u16), // chainedClassSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedClassRecord>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    /// A lookup context record.
    #[derive(Copy)]
    pub LookupRecord { // SequenceLookupRecord
        index        (u16), // sequenceIndex
        lookup_index (u16), // lookupListIndex
    }
}

impl crate::value::Read for Context {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            3 => Self::Format3(tape.take()?),
            value => raise!("found an unknown format of the context table ({value})"),
        })
    }
}

impl crate::value::Read for ChainedContext {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            3 => Self::Format3(tape.take()?),
            value => {
                raise!("found an unknown format of the chained context table ({value})")
            }
        })
    }
}
