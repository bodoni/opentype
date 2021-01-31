use truetype::{GlyphID, Result, Tape, Value, Walue};

use crate::layout::Correction;

/// An anchor.
#[derive(Clone, Debug)]
pub enum Anchor {
    /// Format 1.
    Format1(Anchor1),
    /// Format 2.
    Format2(Anchor2),
    /// Format 3.
    Format3(Anchor3),
}

table! {
    #[doc = "An anchor in format 1."]
    #[derive(Copy)]
    pub Anchor1 { // AnchorFormat1
        format (u16), // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
    }
}

table! {
    #[doc = "An anchor in format 2."]
    #[derive(Copy)]
    pub Anchor2 { // AnchorFormat2
        format (u16), // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
        index  (u16), // AnchorPoint
    }
}

table! {
    @position
    #[doc = "An anchor in format 3."]
    pub Anchor3 { // AnchorFormat3
        format              (u16), // AnchorFormat
        x                   (i16), // XCoordinate
        y                   (i16), // YCoordinate
        x_correction_offset (u16), // XDeviceTable
        y_correction_offset (u16), // YDeviceTable

        x_correction (Option<Correction>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.x_correction_offset)
        },

        y_correction (Option<Correction>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.y_correction_offset)
        },
    }
}

table! {
    @define
    #[doc = "A base attachment."]
    pub Base { // BaseRecord
        anchor_offsets (Vec<u16>   ), // BaseAnchor
        anchors        (Vec<Anchor>),
    }
}

table! {
    @define
    #[doc = "A set of base attachments."]
    pub Bases { // BaseArray
        count   (u16      ), // BaseCount
        records (Vec<Base>), // BaseRecord
    }
}

table! {
    #[doc = "A chaining class positioning rule."]
    pub ChainClassRule { // ChainPosClassRule
        backward_glyph_count (u16), // BacktrackGlyphCount

        backward_class_ids (Vec<u16>) |this, tape| { // Backtrack
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // InputGlyphCount

        input_class_ids (Vec<u16>) |this, tape| { // Input
            if this.input_glyph_count == 0 {
                raise!("found a malformed chaining class positioning rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // LookaheadGlyphCount

        forward_class_ids (Vec<u16>) |this, tape| { // LookAhead
            tape.take_given(this.forward_glyph_count as usize)
        },

        operation_count (u16), // PosCount

        operations (Vec<Positioning>) |this, tape| { // PosLookupRecord
            tape.take_given(this.operation_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of chaining class positioning rules."]
    pub ChainClassRules { // ChainPosClassSet
        count (u16), // ChainPosClassRuleCnt

        offsets (Vec<u16>) |this, tape, _| { // ChainPosClassRule
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainClassRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A chaining positioning rule."]
    pub ChainRule { // ChainPosRule
        backward_glyph_count (u16), // BacktrackGlyphCount

        backward_glyph_ids (Vec<GlyphID>) |this, tape| { // Backtrack
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // InputGlyphCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            if this.input_glyph_count == 0 {
                raise!("found a malformed chaining positioning rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        forward_glyph_count (u16), // LookaheadGlyphCount

        forward_glyph_ids (Vec<GlyphID>) |this, tape| { // LookAhead
            tape.take_given(this.forward_glyph_count as usize)
        },

        operation_count (u16), // PosCount

        operations (Vec<Positioning>) |this, tape| { // PosLookupRecord
            tape.take_given(this.operation_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of chaining positioning rules."]
    pub ChainRules { // ChainPosRuleSet
        count (u16), // ChainPosRuleCount

        offsets (Vec<u16>) |this, tape, _| { // ChainPosRule
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A class positioning rule."]
    pub ClassRule { // PosClassRule
        input_glyph_count (u16), // GlyphCount
        operation_count   (u16), // PosCount

        input_class_ids (Vec<u16>) |this, tape| { // Class
            if this.input_glyph_count == 0 {
                raise!("found a malformed class positioning rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        operations (Vec<Positioning>) |this, tape| { // PosLookupRecord
            tape.take_given(this.operation_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of class positioning rules."]
    pub ClassRules { // PosClassSet
        count (u16), // PosClassRuleCnt

        offsets (Vec<u16>) |this, tape, _| { // PosClassRule
            tape.take_given(this.count as usize)
        },

        records (Vec<ClassRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    @define
    #[doc = "A component attachment."]
    pub Component { // ComponentRecord
        anchor_offsets (Vec<u16>   ),
        anchors        (Vec<Anchor>),
    }
}

table! {
    @define
    #[doc = "A ligature attachment."]
    pub Ligature { // LigatureAttach
        component_count (u16           ), // ComponentCount
        components      (Vec<Component>), // ComponentRecord
    }
}

table! {
    @define
    #[doc = "A set of ligature attachments."]
    pub Ligatures { // LigatureArray
        count   (u16          ), // LigatureCount
        offsets (Vec<u16>     ), // LigatureAttach
        records (Vec<Ligature>),
    }
}

table! {
    @define
    #[doc = "A mark attachment in format 1."]
    pub Mark1 { // MarkRecord
        class_id      (u16   ), // Class
        anchor_offset (u16   ), // MarkAnchor
        anchor        (Anchor),
    }
}

table! {
    @position
    #[doc = "A set of mark attachments in format 1."]
    pub Mark1s { // MarkArray
        count (u16), // MarkCount

        records (Vec<Mark1>) |this, tape, position| { // MarkRecord
            let mut values = Vec::with_capacity(this.count as usize);
            for _ in 0..(this.count as usize) {
                values.push(tape.take_given(position)?);
            }
            Ok(values)
        },
    }
}

table! {
    @define
    #[doc = "A mark attachment in format 2."]
    pub Mark2 { // Mark2Record
        anchor_offsets (Vec<u16>   ), // Mark2Anchor
        anchors        (Vec<Anchor>),
    }
}

table! {
    @define
    #[doc = "A set of mark attachments in format 2."]
    pub Mark2s { // Mark2Array
        count   (u16       ), // Mark2Count
        records (Vec<Mark2>), // Mark2Record
    }
}

table! {
    @define
    #[doc = "A pair adjustment in format 1."]
    pub Pair1 { // PairValueRecord
        glyph2_id (GlyphID), // SecondGlyph
        value1    (Single ), // Value1
        value2    (Single ), // Value2
    }
}

table! {
    @define
    #[doc = "A set of pair adjustments in format 1."]
    pub Pair1s { // PairSet
        count   (u16       ), // PairValueCount
        records (Vec<Pair1>), // PairValueRecord
    }
}

table! {
    @define
    #[doc = "A pair adjustment in format 2."]
    pub Pair2 { // Class2Record
        value1 (Single), // Value1
        value2 (Single), // Value2
    }
}

table! {
    @define
    #[doc = "A set of pair adjustments in format 2."]
    pub Pair2s { // Class1Record
        records (Vec<Pair2>), // Class2Record
    }
}

table! {
    @define
    #[doc = "An entry-exit record."]
    pub Passage { // EntryExitRecord
        entry_offset (u16   ), // EntryAnchor
        exit_offset  (u16   ), // ExitAnchor
        entry        (Anchor),
        exit         (Anchor),
    }
}

table! {
    #[doc = "A positioning record."]
    #[derive(Copy)]
    pub Positioning { // PosLookupRecord
        sequence_index (u16), // SequenceIndex
        lookup_index   (u16), // LookupListIndex
    }
}

table! {
    #[doc = "A positioning rule."]
    pub Rule { // PosRule
        input_glyph_count (u16), // GlyphCount
        operation_count   (u16), // PosCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            if this.input_glyph_count == 0 {
                raise!("found a malformed positioning rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        operations (Vec<Positioning>) |this, tape| { // PosLookupRecord
            tape.take_given(this.operation_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of positioning rules."]
    pub Rules { // PosRuleSet
        count (u16), // PosRuleCount

        offsets (Vec<u16>) |this, tape, _| { // PosRule
            tape.take_given(this.count as usize)
        },

        records (Vec<Rule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    @define
    #[doc = "A single adjustment."]
    pub Single { // ValueRecord
        x_placement                   (Option<i16>), // XPlacement
        y_placement                   (Option<i16>), // YPlacement
        x_advance                     (Option<i16>), // XAdvance
        y_advance                     (Option<i16>), // YAdvance
        x_placement_correction_offset (Option<u16>), // XPlaDevice
        y_placement_correction_offset (Option<u16>), // YPlaDevice
        x_advance_correction_offset   (Option<u16>), // XAdvDevice
        y_advance_correction_offset   (Option<u16>), // YAdvDevice

        x_placement_correction (Option<Correction>),
        y_placement_correction (Option<Correction>),
        x_advance_correction   (Option<Correction>),
        y_advance_correction   (Option<Correction>),
    }
}

flags! {
    #[doc = "Single-adjustment flags."]
    pub SingleFlags(u16) {
        0b0000_0000_0000_0001 => has_x_placement,
        0b0000_0000_0000_0010 => has_y_placement,
        0b0000_0000_0000_0100 => has_x_advance,
        0b0000_0000_0000_1000 => has_y_advance,
        0b0000_0000_0001_0000 => has_x_placement_correction,
        0b0000_0000_0010_0000 => has_y_placement_correction,
        0b0000_0000_0100_0000 => has_x_advance_correction,
        0b0000_0000_1000_0000 => has_y_advance_correction,
        0b1111_1111_0000_0000 => is_invalid,
    }
}

impl Default for Anchor {
    #[inline]
    fn default() -> Self {
        Anchor::Format1(Anchor1::default())
    }
}

impl Value for Anchor {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Anchor::Format1(tape.take()?),
            2 => Anchor::Format2(tape.take()?),
            3 => Anchor::Format3(tape.take()?),
            _ => raise!("found an unknown format of the anchor table"),
        })
    }
}

impl Walue<'static> for Base {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(class_count as usize)?;
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Base {
            anchor_offsets: anchor_offsets,
            anchors: anchors,
        })
    }
}

impl Walue<'static> for Bases {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(tape.take_given((position, class_count))?);
        }
        Ok(Bases {
            count: count,
            records: records,
        })
    }
}

impl Walue<'static> for Component {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(class_count as usize)?;
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Component {
            anchor_offsets: anchor_offsets,
            anchors: anchors,
        })
    }
}

impl Walue<'static> for Ligature {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = tape.position()?;
        let component_count = tape.take()?;
        let mut components = Vec::with_capacity(component_count as usize);
        for _ in 0..(component_count as usize) {
            components.push(tape.take_given((position, class_count))?);
        }
        Ok(Ligature {
            component_count: component_count,
            components: components,
        })
    }
}

impl Walue<'static> for Ligatures {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let offsets: Vec<u16> = tape.take_given(count as usize)?;
        let records = jump_take_given!(@unwrap tape, position, count, offsets, class_count);
        Ok(Ligatures {
            count: count,
            offsets: offsets,
            records: records,
        })
    }
}

impl Walue<'static> for Mark1 {
    type Parameter = u64;

    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let class_id = tape.take()?;
        let anchor_offset = tape.take()?;
        let anchor = jump_take!(@unwrap tape, position, anchor_offset);
        Ok(Mark1 {
            class_id: class_id,
            anchor_offset: anchor_offset,
            anchor: anchor,
        })
    }
}

impl Walue<'static> for Mark2 {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(class_count as usize)?;
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Mark2 {
            anchor_offsets: anchor_offsets,
            anchors: anchors,
        })
    }
}

impl Walue<'static> for Mark2s {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(tape.take_given((position, class_count))?);
        }
        Ok(Mark2s {
            count: count,
            records: records,
        })
    }
}

impl Walue<'static> for Pair1 {
    type Parameter = (u64, SingleFlags, SingleFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        Ok(Pair1 {
            glyph2_id: tape.take()?,
            value1: tape.take_given((position, value1_flags))?,
            value2: tape.take_given((position, value2_flags))?,
        })
    }
}

impl Walue<'static> for Pair1s {
    type Parameter = (u64, SingleFlags, SingleFlags);

    fn read<T: Tape>(tape: &mut T, parameter: Self::Parameter) -> Result<Self> {
        let count = tape.take()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(tape.take_given(parameter)?);
        }
        Ok(Pair1s {
            count: count,
            records: records,
        })
    }
}

impl Walue<'static> for Pair2 {
    type Parameter = (u64, SingleFlags, SingleFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        Ok(Pair2 {
            value1: tape.take_given((position, value1_flags))?,
            value2: tape.take_given((position, value2_flags))?,
        })
    }
}

impl Walue<'static> for Pair2s {
    type Parameter = (u64, u16, SingleFlags, SingleFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, class2_count, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        let mut records = Vec::with_capacity(class2_count as usize);
        for _ in 0..(class2_count as usize) {
            records.push(tape.take_given((position, value1_flags, value2_flags))?);
        }
        Ok(Pair2s { records: records })
    }
}

impl Walue<'static> for Passage {
    type Parameter = u64;

    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let entry_offset = tape.take()?;
        let exit_offset = tape.take()?;
        let entry = jump_take!(@unwrap tape, position, entry_offset);
        let exit = jump_take!(@unwrap tape, position, exit_offset);
        Ok(Passage {
            entry_offset: entry_offset,
            exit_offset: exit_offset,
            entry: entry,
            exit: exit,
        })
    }
}

impl Walue<'static> for Single {
    type Parameter = (u64, SingleFlags);

    fn read<T: Tape>(tape: &mut T, (position, flags): Self::Parameter) -> Result<Self> {
        macro_rules! take(
            ($flag:ident) => (if flags.$flag() { Some(tape.take()?) } else { None });
        );
        let x_placement = take!(has_x_placement);
        let y_placement = take!(has_y_placement);
        let x_advance = take!(has_x_advance);
        let y_advance = take!(has_y_advance);
        let x_placement_correction_offset = take!(has_x_placement_correction);
        let y_placement_correction_offset = take!(has_y_placement_correction);
        let x_advance_correction_offset = take!(has_x_advance_correction);
        let y_advance_correction_offset = take!(has_y_advance_correction);
        macro_rules! take(
            ($offset:ident) => (match $offset {
                Some(offset) => Some(jump_take!(@unwrap tape, position, offset)),
                _ => None,
            });
        );
        let x_placement_correction = take!(x_placement_correction_offset);
        let y_placement_correction = take!(y_placement_correction_offset);
        let x_advance_correction = take!(x_advance_correction_offset);
        let y_advance_correction = take!(y_advance_correction_offset);
        Ok(Single {
            x_placement: x_placement,
            y_placement: y_placement,
            x_advance: x_advance,
            y_advance: y_advance,
            x_placement_correction_offset: x_placement_correction_offset,
            y_placement_correction_offset: y_placement_correction_offset,
            x_advance_correction_offset: x_advance_correction_offset,
            y_advance_correction_offset: y_advance_correction_offset,
            x_placement_correction: x_placement_correction,
            y_placement_correction: y_placement_correction,
            x_advance_correction: x_advance_correction,
            y_advance_correction: y_advance_correction,
        })
    }
}
