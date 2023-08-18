use truetype::GlyphID;

use crate::layout::Correction;
use crate::{Result, Tape, Walue};

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
        format (u16), // anchorFormat
        x      (i16), // xCoordinate
        y      (i16), // yCoordinate
    }
}

table! {
    #[doc = "An anchor in format 2."]
    #[derive(Copy)]
    pub Anchor2 { // AnchorFormat2
        format (u16), // anchorFormat
        x      (i16), // xCoordinate
        y      (i16), // yCoordinate
        index  (u16), // anchorPoint
    }
}

table! {
    @position
    #[doc = "An anchor in format 3."]
    pub Anchor3 { // AnchorFormat3
        format              (u16), // anchorFormat
        x                   (i16), // xCoordinate
        y                   (i16), // yCoordinate
        x_correction_offset (u16), // xDeviceOffset
        y_correction_offset (u16), // yDeviceOffset

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
        anchor_offsets (Vec<u16>), // baseAnchorOffsets
    }
}

table! {
    @define
    #[doc = "A set of base attachments."]
    pub Bases { // BaseArray
        count   (u16      ), // baseCount
        records (Vec<Base>), // baseRecords

        anchors (Vec<Vec<Option<Anchor>>>),
    }
}

table! {
    @define
    #[doc = "A component attachment."]
    pub Component { // ComponentRecord
        anchor_offsets (Vec<u16>), // ligatureAnchorOffsets
    }
}

table! {
    @define
    #[doc = "A ligature attachment."]
    pub Ligature { // LigatureAttach
        count   (u16           ), // componentCount
        records (Vec<Component>), // componentRecords
    }
}

table! {
    @define
    #[doc = "A set of ligature attachments."]
    pub Ligatures { // LigatureArray
        count   (u16     ), // ligatureCount
        offsets (Vec<u16>), // ligatureAttachOffsets

        records (Vec<Ligature>),
        anchors (Vec<Vec<Vec<Option<Anchor>>>>),
    }
}

table! {
    #[doc = "A mark attachment in format 1."]
    pub Mark1 { // MarkRecord
        class_id      (u16), // markClass
        anchor_offset (u16), // markAnchorOffset
    }
}

table! {
    @position
    #[doc = "A set of mark attachments in format 1."]
    pub Mark1s { // MarkArray
        count (u16), // markCount

        records (Vec<Mark1>) |this, tape, _| { // markRecords
            tape.take_given(this.count as usize)
        },

        anchors (Vec<Anchor>) |this, tape, position| {
            jump_take!(tape, position, this.count, i => this.records[i].anchor_offset)
        },
    }
}

table! {
    @define
    #[doc = "A mark attachment in format 2."]
    pub Mark2 { // Mark2Record
        anchor_offsets (Vec<u16>), // mark2AnchorOffsets
    }
}

table! {
    @define
    #[doc = "A set of mark attachments in format 2."]
    pub Mark2s { // Mark2Array
        count   (u16       ), // mark2Count
        records (Vec<Mark2>), // mark2Records

        anchors (Vec<Vec<Option<Anchor>>>),
    }
}

table! {
    @define
    #[doc = "A pair adjustment in format 1."]
    pub Pair1 { // PairValueRecord
        glyph2_id (GlyphID), // secondGlyph
        value1    (Option<Value>), // valueRecord1
        value2    (Option<Value>), // valueRecord2
    }
}

table! {
    @define
    #[doc = "A set of pair adjustments in format 1."]
    pub Pair1s { // PairSet
        count   (u16       ), // pairValueCount
        records (Vec<Pair1>), // pairValueRecords
    }
}

table! {
    @define
    #[doc = "A pair adjustment in format 2."]
    pub Pair2 { // Class2Record
        value1 (Option<Value>), // valueRecord1
        value2 (Option<Value>), // valueRecord2
    }
}

table! {
    @define
    #[doc = "A set of pair adjustments in format 2."]
    pub Pair2s { // Class1Record
        records (Vec<Pair2>), // class2Records
    }
}

table! {
    @define
    #[doc = "An entry–exit record."]
    pub Passage { // EntryExitRecord
        entry_offset (u16), // entryAnchorOffset
        exit_offset  (u16), // exitAnchorOffset

        entry (Option<Anchor>),
        exit  (Option<Anchor>),
    }
}

table! {
    @define
    #[doc = "A single adjustment."]
    pub Value { // ValueRecord
        x_placement                   (Option<i16>), // xPlacement
        y_placement                   (Option<i16>), // yPlacement
        x_advance                     (Option<i16>), // xAdvance
        y_advance                     (Option<i16>), // yAdvance
        x_placement_correction_offset (Option<u16>), // xPlaDeviceOffset
        y_placement_correction_offset (Option<u16>), // yPlaDeviceOffset
        x_advance_correction_offset   (Option<u16>), // xAdvDeviceOffset
        y_advance_correction_offset   (Option<u16>), // yAdvDeviceOffset

        x_placement_correction (Option<Correction>),
        y_placement_correction (Option<Correction>),
        x_advance_correction   (Option<Correction>),
        y_advance_correction   (Option<Correction>),
    }
}

flags! {
    #[doc = "Value flags."]
    pub ValueFlags(u16) { // ValueFormat
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

impl crate::Value for Anchor {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Anchor::Format1(tape.take()?),
            2 => Anchor::Format2(tape.take()?),
            3 => Anchor::Format3(tape.take()?),
            value => raise!("found an unknown format of the anchor table ({value})"),
        })
    }
}

impl Walue<'static> for Base {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let anchor_offsets = tape.take_given(mark_class_count as usize)?;
        Ok(Self { anchor_offsets })
    }
}

impl Walue<'static> for Bases {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let mut records: Vec<Base> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(tape.take_given(mark_class_count)?);
        }
        let mut anchors = Vec::with_capacity(count as usize);
        #[allow(clippy::needless_range_loop)]
        for i in 0..(count as usize) {
            anchors.push(jump_take_maybe!(
                @unwrap
                tape,
                position,
                mark_class_count,
                j => records[i].anchor_offsets[j]
            ));
        }
        Ok(Self {
            count,
            records,
            anchors,
        })
    }
}

impl Walue<'static> for Component {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let anchor_offsets = tape.take_given(mark_class_count as usize)?;
        Ok(Self { anchor_offsets })
    }
}

impl Walue<'static> for Ligature {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let count = tape.take()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(tape.take_given(mark_class_count)?);
        }
        Ok(Self { count, records })
    }
}

impl Walue<'static> for Ligatures {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let offsets: Vec<u16> = tape.take_given(count as usize)?;
        let records: Vec<Ligature> =
            jump_take_given!(@unwrap tape, position, count, offsets, mark_class_count);
        let mut anchors = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let mut inner = Vec::with_capacity(records[i].count as usize);
            for j in 0..(records[i].count as usize) {
                inner.push(jump_take_maybe!(
                    @unwrap
                    tape,
                    position,
                    mark_class_count,
                    k => {
                        let offset = records[i].records[j].anchor_offsets[k];
                        if offset > 0 { offsets[i] + offset } else { 0 }
                    }
                ));
            }
            anchors.push(inner);
        }
        Ok(Self {
            count,
            offsets,
            records,
            anchors,
        })
    }
}

impl Walue<'static> for Mark2 {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let anchor_offsets = tape.take_given(mark_class_count as usize)?;
        Ok(Self { anchor_offsets })
    }
}

impl Walue<'static> for Mark2s {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take::<u16>()?;
        let mut records: Vec<Mark2> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(tape.take_given(mark_class_count)?);
        }
        let mut anchors = Vec::with_capacity(count as usize);
        #[allow(clippy::needless_range_loop)]
        for i in 0..(count as usize) {
            anchors.push(jump_take_maybe!(
                @unwrap
                tape,
                position,
                mark_class_count,
                j => records[i].anchor_offsets[j]
            ));
        }
        Ok(Self {
            count,
            records,
            anchors,
        })
    }
}

impl Walue<'static> for Pair1 {
    type Parameter = (u64, ValueFlags, ValueFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        Ok(Self {
            glyph2_id: tape.take()?,
            value1: if value1_flags.0 > 0 {
                Some(tape.take_given((position, value1_flags))?)
            } else {
                None
            },
            value2: if value2_flags.0 > 0 {
                Some(tape.take_given((position, value2_flags))?)
            } else {
                None
            },
        })
    }
}

impl Walue<'static> for Pair1s {
    type Parameter = (ValueFlags, ValueFlags);

    fn read<T: Tape>(tape: &mut T, (value1_flags, value2_flags): Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(tape.take_given((position, value1_flags, value2_flags))?);
        }
        Ok(Self { count, records })
    }
}

impl Walue<'static> for Pair2 {
    type Parameter = (u64, ValueFlags, ValueFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        Ok(Self {
            value1: if value1_flags.0 > 0 {
                Some(tape.take_given((position, value1_flags))?)
            } else {
                None
            },
            value2: if value2_flags.0 > 0 {
                Some(tape.take_given((position, value2_flags))?)
            } else {
                None
            },
        })
    }
}

impl Walue<'static> for Pair2s {
    type Parameter = (u64, u16, ValueFlags, ValueFlags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, class2_count, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        let mut records = Vec::with_capacity(class2_count as usize);
        for _ in 0..(class2_count as usize) {
            records.push(tape.take_given((position, value1_flags, value2_flags))?);
        }
        Ok(Self { records })
    }
}

impl Walue<'static> for Passage {
    type Parameter = u64;

    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let entry_offset = tape.take()?;
        let exit_offset = tape.take()?;
        let entry = jump_take_maybe!(@unwrap tape, position, entry_offset);
        let exit = jump_take_maybe!(@unwrap tape, position, exit_offset);
        Ok(Self {
            entry_offset,
            exit_offset,
            entry,
            exit,
        })
    }
}

impl Walue<'static> for Value {
    type Parameter = (u64, ValueFlags);

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
        let (
            x_placement_correction,
            y_placement_correction,
            x_advance_correction,
            y_advance_correction,
        ) = tape.stay(|tape| {
            macro_rules! take(
                ($offset:ident) => (match $offset {
                    Some(offset) => jump_take_maybe!(@unwrap tape, position, offset),
                    _ => None,
                });
            );
            Ok((
                take!(x_placement_correction_offset),
                take!(y_placement_correction_offset),
                take!(x_advance_correction_offset),
                take!(y_advance_correction_offset),
            ))
        })?;
        Ok(Self {
            x_placement,
            y_placement,
            x_advance,
            y_advance,
            x_placement_correction_offset,
            y_placement_correction_offset,
            x_advance_correction_offset,
            y_advance_correction_offset,
            x_placement_correction,
            y_placement_correction,
            x_advance_correction,
            y_advance_correction,
        })
    }
}
