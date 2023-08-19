use truetype::GlyphID;

use crate::layout::Correction;
use crate::{Result, Tape, Value, Walue};

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
    #[doc = "A base."]
    pub Base { // BaseRecord
        anchor_offsets (Vec<u16>), // baseAnchorOffsets

        anchors (Vec<Option<Anchor>>),
    }
}

table! {
    @define
    #[doc = "Bases."]
    pub Bases { // BaseArray
        count   (u16      ), // baseCount
        records (Vec<Base>), // baseRecords
    }
}

table! {
    @define
    #[doc = "A component."]
    pub Component { // ComponentRecord
        anchor_offsets (Vec<u16>), // ligatureAnchorOffsets

        anchors (Vec<Option<Anchor>>),
    }
}

table! {
    @define
    #[doc = "A connection."]
    pub Connection { // EntryExitRecord
        start_anchor_offset (u16), // entryAnchorOffset
        end_anchor_offset   (u16), // exitAnchorOffset

        start_anchor (Option<Anchor>),
        end_anchor   (Option<Anchor>),
    }
}

table! {
    @define
    #[doc = "A ligature."]
    pub Ligature { // LigatureAttach
        count      (u16           ), // componentCount
        components (Vec<Component>), // componentRecords
    }
}

table! {
    @define
    #[doc = "Ligatures."]
    pub Ligatures { // LigatureArray
        count   (u16     ), // ligatureCount
        offsets (Vec<u16>), // ligatureAttachOffsets

        records (Vec<Ligature>),
    }
}

table! {
    @define
    #[doc = "A mark in format 1."]
    pub Mark1 { // MarkRecord
        class_id      (u16), // markClass
        anchor_offset (u16), // markAnchorOffset

        anchor (Anchor),
    }
}

table! {
    @position
    #[doc = "Marks in format 1."]
    pub Mark1s { // MarkArray
        count (u16), // markCount

        records (Vec<Mark1>) |this, tape, position| { // markRecords
            (0..this.count).map(|_| tape.take_given(position)).collect()
        },
    }
}

table! {
    @define
    #[doc = "A mark in format 2."]
    pub Mark2 { // Mark2Record
        anchor_offsets (Vec<u16>), // mark2AnchorOffsets

        anchors (Vec<Option<Anchor>>),
    }
}

table! {
    @define
    #[doc = "Marks in format 2."]
    pub Mark2s { // Mark2Array
        count   (u16       ), // mark2Count
        records (Vec<Mark2>), // mark2Records
    }
}

table! {
    @define
    #[doc = "A value pair in format 1."]
    pub Pair1 { // PairValueRecord
        glyph2_id (GlyphID       ), // secondGlyph
        value1    (Option<Single>), // valueRecord1
        value2    (Option<Single>), // valueRecord2
    }
}

table! {
    @define
    #[doc = "Pairs of values in format 1."]
    pub Pair1s { // PairSet
        count   (u16       ), // pairValueCount
        records (Vec<Pair1>), // pairValueRecords
    }
}

table! {
    @define
    #[doc = "A value pair in format 2."]
    pub Pair2 { // Class2Record
        value1 (Option<Single>), // valueRecord1
        value2 (Option<Single>), // valueRecord2
    }
}

table! {
    @define
    #[doc = "Value pairs in format 2."]
    pub Pair2s { // Class1Record
        records (Vec<Pair2>), // class2Records
    }
}

table! {
    @define
    #[doc = "A single value."]
    pub Single { // ValueRecord
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
    #[doc = "Adjustment flags."]
    pub Flags(u16) { // ValueFormat
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
            value => raise!("found an unknown format of the anchor table ({value})"),
        })
    }
}

impl Walue<'static> for Base {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, mark_class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(mark_class_count as usize)?;
        let anchors =
            tape.stay(|tape| jump_take_maybe!(tape, position, mark_class_count, anchor_offsets))?;
        Ok(Self {
            anchor_offsets,
            anchors,
        })
    }
}

impl Walue<'static> for Bases {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let records = (0..count)
            .map(|_| tape.take_given((position, mark_class_count)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { count, records })
    }
}

impl Walue<'static> for Component {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, mark_class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(mark_class_count as usize)?;
        let anchors =
            tape.stay(|tape| jump_take_maybe!(tape, position, mark_class_count, anchor_offsets))?;
        Ok(Self {
            anchor_offsets,
            anchors,
        })
    }
}

impl Walue<'static> for Ligature {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let components = (0..count)
            .map(|_| tape.take_given((position, mark_class_count)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { count, components })
    }
}

impl Walue<'static> for Ligatures {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let offsets: Vec<u16> = tape.take_given(count as usize)?;
        let records =
            tape.stay(|tape| jump_take_given!(tape, position, count, offsets, mark_class_count))?;
        Ok(Self {
            count,
            offsets,
            records,
        })
    }
}

impl Walue<'static> for Mark1 {
    type Parameter = u64;

    fn read<T: Tape>(tape: &mut T, position: Self::Parameter) -> Result<Self> {
        let class_id = tape.take()?;
        let anchor_offset = tape.take()?;
        let anchor = tape.stay(|tape| jump_take!(tape, position, anchor_offset))?;
        Ok(Self {
            class_id,
            anchor_offset,
            anchor,
        })
    }
}

impl Walue<'static> for Mark2 {
    type Parameter = (u64, u16);

    fn read<T: Tape>(tape: &mut T, (position, mark_class_count): Self::Parameter) -> Result<Self> {
        let anchor_offsets: Vec<u16> = tape.take_given(mark_class_count as usize)?;
        let anchors =
            tape.stay(|tape| jump_take_maybe!(tape, position, mark_class_count, anchor_offsets))?;
        Ok(Self {
            anchor_offsets,
            anchors,
        })
    }
}

impl Walue<'static> for Mark2s {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, mark_class_count: Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let records = (0..count)
            .map(|_| tape.take_given((position, mark_class_count)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { count, records })
    }
}

impl Walue<'static> for Pair1 {
    type Parameter = (u64, Flags, Flags);

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
    type Parameter = (Flags, Flags);

    fn read<T: Tape>(tape: &mut T, (value1_flags, value2_flags): Self::Parameter) -> Result<Self> {
        let position = tape.position()?;
        let count = tape.take()?;
        let records = (0..count)
            .map(|_| tape.take_given((position, value1_flags, value2_flags)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { count, records })
    }
}

impl Walue<'static> for Pair2 {
    type Parameter = (u64, Flags, Flags);

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
    type Parameter = (u64, u16, Flags, Flags);

    fn read<T: Tape>(
        tape: &mut T,
        (position, class2_count, value1_flags, value2_flags): Self::Parameter,
    ) -> Result<Self> {
        let records = (0..class2_count)
            .map(|_| tape.take_given((position, value1_flags, value2_flags)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { records })
    }
}

impl Walue<'static> for Connection {
    type Parameter = u64;

    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let start_anchor_offset = tape.take()?;
        let end_anchor_offset = tape.take()?;
        let start_anchor = jump_take_maybe!(@unwrap tape, position, start_anchor_offset);
        let end_anchor = jump_take_maybe!(@unwrap tape, position, end_anchor_offset);
        Ok(Self {
            start_anchor_offset,
            end_anchor_offset,
            start_anchor,
            end_anchor,
        })
    }
}

impl Walue<'static> for Single {
    type Parameter = (u64, Flags);

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
