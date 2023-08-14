use truetype::GlyphID;

use crate::layout::{Class, Coverage};
use crate::{Result, Tape, Value};

/// A table for contextual lookup.
#[derive(Clone, Debug)]
pub enum SequenceContext {
    /// Format 1.
    Format1(SequenceContext1),
    /// Format 2.
    Format2(SequenceContext2),
    /// Format 3.
    Format3(SequenceContext3),
}

table! {
    @position
    #[doc = "A table for contextual lookup in format 1."]
    pub SequenceContext1 { // SequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        rule_count      (u16), // seqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // seqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<SequenceRules>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for contextual lookup in format 2."]
    pub SequenceContext2 { // SequenceContextFormat2
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

        rules (Vec<Option<ClassSequenceRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for contextual lookup in format 3."]
    pub SequenceContext3 { // SequenceContextFormat3
        format       (u16), // format
        glyph_count  (u16), // glyphCount
        lookup_count (u16), // seqLookupCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // coverageOffsets
            tape.take_given(this.glyph_count as usize)
        },

        lookups (Vec<SequenceLookup>) |this, tape, _| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

/// A table for chained contextual lookup.
#[derive(Clone, Debug)]
pub enum ChainedSequenceContext {
    /// Format 1.
    Format1(ChainedSequenceContext1),
    /// Format 2.
    Format2(ChainedSequenceContext2),
    /// Format 3.
    Format3(ChainedSequenceContext3),
}

table! {
    @position
    #[doc = "A table for chained contextual lookup in format 1."]
    pub ChainedSequenceContext1 { // ChainedSequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        rule_count      (u16), // chainedSeqRuleSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<ChainedSequenceRules>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for chained contextual lookup in format 2."]
    pub ChainedSequenceContext2 { // ChainedSequenceContextFormat2
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

        rules (Vec<Option<ChainedClassSequenceRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for chained contextual lookup in format 3."]
    pub ChainedSequenceContext3 { // ChainedSequenceContextFormat3
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

        lookup_count (u16), // seqLookupCount

        lookups (Vec<SequenceLookup>) |this, tape, _| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
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
    #[doc = "A sequence rule."]
    pub SequeceRule { // SequenceRule
        input_glyph_count (u16), // glyphCount
        lookup_count      (u16), // lookupCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed sequence rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        lookups (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of sequence rules."]
    pub SequenceRules { // SequenceRuleSet
        count (u16), // seqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // seqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<SequeceRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A class sequence rule."]
    pub ClassSequeceRule { // ClassSequenceRule
        input_glyph_count (u16), // glyphCount
        lookup_count   (u16), // seqLookupCount

        input_class_ids (Vec<u16>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed class sequence rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        lookups (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of class sequence rules."]
    pub ClassSequenceRules { // ClassSequenceRuleSet
        count (u16), // classSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // classSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ClassSequeceRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A chained sequence rule."]
    pub ChainedSequenceRule { // ChainedSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_glyph_ids (Vec<GlyphID>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained sequence rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_glyph_ids (Vec<GlyphID>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        lookup_count (u16), // seqLookupCount

        lookups (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of chained sequence rules."]
    pub ChainedSequenceRules { // ChainedSequenceRuleSet
        count (u16), // chainedSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedSequenceRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A chained class sequence rule."]
    pub ChainedClassSequenceRule { // ChainedClassSequenceRule
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_class_ids (Vec<u16>) |this, tape| { // backtrackSequence
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_class_ids (Vec<u16>) |this, tape| { // inputSequence
            if this.input_glyph_count == 0 {
                raise!("found a malformed chained class sequence rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_class_ids (Vec<u16>) |this, tape| { // lookaheadSequence
            tape.take_given(this.forward_glyph_count as usize)
        },

        lookup_count (u16), // seqLookupCount

        lookups (Vec<SequenceLookup>) |this, tape| { // seqLookupRecords
            tape.take_given(this.lookup_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of chained class sequence rules."]
    pub ChainedClassSequenceRules { // ChainedClassSequenceRuleSet
        count (u16), // chainedClassSeqRuleCount

        offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainedClassSequenceRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A sequence lookup."]
    #[derive(Copy)]
    pub SequenceLookup { // SequenceLookupRecord
        sequence_index (u16), // sequenceIndex
        lookup_index   (u16), // lookupListIndex
    }
}

impl Value for SequenceContext {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            3 => Self::Format3(tape.take()?),
            value => raise!("found an unknown format of the sequence-context table ({value})"),
        })
    }
}

impl Value for ChainedSequenceContext {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            3 => Self::Format3(tape.take()?),
            value => {
                raise!("found an unknown format of the chained sequence-context table ({value})")
            }
        })
    }
}
