//! The lookup context.

use truetype::GlyphID;

use crate::layout::{Class, Coverage};
use crate::{Result, Tape, Value};

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
    #[doc = "A contextual lookup in format 1."]
    pub Context1 { // SequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        rule_count      (u16), // seqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // seqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<Rules>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A contextual lookup in format 2."]
    pub Context2 { // SequenceContextFormat2
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        class_offset    (u16), // classDefOffset
        rule_count      (u16), // classSeqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // classSeqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<Option<ClassRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A contextual lookup in format 3."]
    pub Context3 { // SequenceContextFormat3
        format       (u16), // format
        glyph_count  (u16), // glyphCount
        record_count (u16), // seqLookupCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // coverageOffsets
            tape.take_given(this.glyph_count as usize)
        },

        records (Vec<SequenceLookup>) |this, tape, _| { // seqLookupRecords
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
    #[doc = "A chained contextual lookup in format 1."]
    pub ChainedContext1 { // ChainedSequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        rule_count      (u16), // chainedSeqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<ChainedRules>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A chained contextual lookup in format 2."]
    pub ChainedContext2 { // ChainedSequenceContextFormat2
        format                (u16), // format
        coverage_offset       (u16), // coverageOffset
        backward_class_offset (u16), // backtrackClassDefOffset
        input_class_offset    (u16), // inputClassDefOffset
        forward_class_offset  (u16), // lookaheadClassDefOffset
        rule_count            (u16), // chainedClassSeqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
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

        rules (Vec<Option<ChainedClassRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A chained contextual lookup in format 3."]
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

        records (Vec<SequenceLookup>) |this, tape, _| { // seqLookupRecords
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
    #[doc = "A rule."]
    pub Rule { // SequenceRule
        input_glyph_count (u16), // glyphCount
        record_count      (u16), // lookupCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        records (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "Rules."]
    pub Rules { // SequenceRuleSet
        count (u16), // seqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // seqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<Rule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A class rule."]
    pub ClassRule { // ClassSequenceRule
        input_glyph_count (u16), // glyphCount
        record_count   (u16), // seqLookupCount

        input_class_ids (Vec<u16>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed class rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        records (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "Class rules."]
    pub ClassRules { // ClassSequenceRuleSet
        count (u16), // classSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // classSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ClassRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A chained rule."]
    pub ChainedRule { // ChainedSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_glyph_ids (Vec<GlyphID>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_glyph_ids (Vec<GlyphID>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        record_count (u16), // seqLookupCount

        records (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "Chained rules."]
    pub ChainedRules { // ChainedSequenceRuleSet
        count (u16), // chainedSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A chained class rule."]
    pub ChainedClassRule { // ChainedClassSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_class_ids (Vec<u16>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_class_ids (Vec<u16>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained class rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_class_ids (Vec<u16>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        record_count (u16), // seqLookupCount

        records (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "Chained class rules."]
    pub ChainedClassRules { // ChainedClassSequenceRuleSet
        count (u16), // chainedClassSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedClassRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A sequence–lookup."]
    #[derive(Copy)]
    pub SequenceLookup { // SequenceLookupRecord
        sequence_index (u16), // sequenceIndex
        lookup_index   (u16), // lookupListIndex
    }
}

impl Value for Context {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            3 => Self::Format3(tape.take()?),
            value => raise!("found an unknown format of the context table ({value})"),
        })
    }
}

impl Value for ChainedContext {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
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
