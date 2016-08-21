#![allow(unused_mut, unused_variables)]

use truetype::GlyphID;

use {Result, Tape, Value, Walue};

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
        format (u16) = { 1 }, // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
    }
}

table! {
    #[doc = "An anchor in format 2."]
    #[derive(Copy)]
    pub Anchor2 { // AnchorFormat2
        format (u16) = { 2 }, // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
        index  (u16), // AnchorPoint
    }
}

table! {
    @position
    #[doc = "An anchor in format 3."]
    pub Anchor3 { // AnchorFormat3
        format          (u16) = { 3 }, // AnchorFormat
        x               (i16), // XCoordinate
        y               (i16), // YCoordinate
        device_x_offset (u16), // XDeviceTable
        device_y_offset (u16), // YDeviceTable

        device_x (Option<Device>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.device_x_offset)
        },

        device_y (Option<Device>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.device_y_offset)
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
    pub BaseSet { // BaseArray
        count   (u16      ), // BaseCount
        records (Vec<Base>), // BaseRecord
    }
}

table! {
    @define
    #[doc = "A device adjustment."]
    pub Device { // Device
        start_size   (u16     ), // StartSize
        end_size     (u16     ), // EndSize
        delta_format (u16     ), // DeltaFormat
        delta_data   (Vec<u16>), // DeltaValue
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
    pub LigatureSet { // LigatureArray
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
    pub Mark1Set { // MarkArray
        count (u16), // MarkCount

        records (Vec<Mark1>) |this, tape, position| { // MarkRecord
            let mut values = Vec::with_capacity(this.count as usize);
            for i in 0..(this.count as usize) {
                values.push(try!(tape.take_given(position)));
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
    pub Mark2Set { // Mark2Array
        count   (u16       ), // Mark2Count
        records (Vec<Mark2>), // Mark2Record
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
    @define
    #[doc = "A single adjustment."]
    pub Single { // ValueRecord
        x_placement               (Option<i16>   ), // XPlacement
        y_placement               (Option<i16>   ), // YPlacement
        x_advance                 (Option<i16>   ), // XAdvance
        y_advance                 (Option<i16>   ), // YAdvance
        device_x_placement_offset (Option<u16>   ), // XPlaDevice
        device_y_placement_offset (Option<u16>   ), // YPlaDevice
        device_x_advance_offset   (Option<u16>   ), // XAdvDevice
        device_y_advance_offset   (Option<u16>   ), // YAdvDevice
        device_x_placement        (Option<Device>),
        device_y_placement        (Option<Device>),
        device_x_advance          (Option<Device>),
        device_y_advance          (Option<Device>),
    }
}

flags! {
    #[doc = "Single-adjustment flags."]
    pub SingleFlags(u16) {
        0b0000_0000_0000_0001 => has_x_placement,
        0b0000_0000_0000_0010 => has_y_placement,
        0b0000_0000_0000_0100 => has_x_advance,
        0b0000_0000_0000_1000 => has_y_advance,
        0b0000_0000_0001_0000 => has_device_x_placement,
        0b0000_0000_0010_0000 => has_device_y_placement,
        0b0000_0000_0100_0000 => has_device_x_advance,
        0b0000_0000_1000_0000 => has_device_y_advance,
        0b1111_1111_0000_0000 => is_invalid,
    }
}

table! {
    @define
    #[doc = "A pair adjustment."]
    pub Pair { // PairValueRecord
        value1 (Single), // Value1
        value2 (Single), // Value2
    }
}

table! {
    @define
    #[doc = "A set of pair adjustments."]
    pub PairSet { // PairSet
        count   (u16      ), // PairValueCount
        records (Vec<Pair>), // PairValueRecord
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
        positioning_count (u16), // PosCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            if this.input_glyph_count == 0 {
                raise!("found a malformed positioning rule");
            }
            tape.take_given(this.input_glyph_count as usize - 1)
        },

        positionings (Vec<Positioning>) |this, tape| { // PosLookupRecord
            tape.take_given(this.positioning_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of positioning rules."]
    pub RuleSet { // PosRuleSet
        count (u16), // PosRuleCount

        offsets (Vec<u16>) |this, tape, _| { // PosRule
            tape.take_given(this.count as usize)
        },

        records (Vec<Rule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

impl Value for Anchor {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Anchor::Format1(try!(tape.take())),
            2 => Anchor::Format2(try!(tape.take())),
            3 => Anchor::Format3(try!(tape.take())),
            _ => raise!("found an unknown format of the anchor table"),
        })
    }
}

impl Walue<(u64, u16)> for Base {
    fn read<T: Tape>(tape: &mut T, (position, class_count): (u64, u16)) -> Result<Self> {
        let anchor_offsets: Vec<u16> = try!(tape.take_given(class_count as usize));
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Base { anchor_offsets: anchor_offsets, anchors: anchors })
    }
}

impl Walue<u16> for BaseSet {
    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            records.push(try!(tape.take_given((position, class_count))));
        }
        Ok(BaseSet { count: count, records: records })
    }
}

impl Walue<(u64, u16)> for Component {
    fn read<T: Tape>(tape: &mut T, (position, class_count): (u64, u16)) -> Result<Self> {
        let anchor_offsets: Vec<u16> = try!(tape.take_given(class_count as usize));
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Component { anchor_offsets: anchor_offsets, anchors: anchors })
    }
}

impl Value for Device {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let start_size = try!(tape.take());
        let end_size = try!(tape.take());
        if start_size > end_size {
            raise!("found a malformed device table");
        }
        let delta_format = try!(tape.take());
        if delta_format == 0 || delta_format > 3 {
            raise!("found an unknown format of the device table");
        }
        let count = (end_size - start_size) as usize + 1;
        let bit_count = (1 << delta_format as usize) * count;
        let short_count = (bit_count + 16 - bit_count % 16) >> 4;
        let delta_data = try!(tape.take_given(short_count));
        Ok(Device {
            start_size: start_size,
            end_size: end_size,
            delta_format: delta_format,
            delta_data: delta_data,
        })
    }
}

impl Walue<u16> for Ligature {
    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = try!(tape.position());
        let component_count = try!(tape.take());
        let mut components = Vec::with_capacity(component_count as usize);
        for i in 0..(component_count as usize) {
            components.push(try!(tape.take_given((position, class_count))));
        }
        Ok(Ligature { component_count: component_count, components: components })
    }
}

impl Walue<u16> for LigatureSet {
    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take());
        let offsets: Vec<u16> = try!(tape.take_given(count as usize));
        let records = jump_take_given!(@unwrap tape, position, count, offsets, class_count);
        Ok(LigatureSet { count: count, offsets: offsets, records: records })
    }
}

impl Walue<u64> for Mark1 {
    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let class_id = try!(tape.take());
        let anchor_offset = try!(tape.take());
        let anchor = jump_take!(@unwrap tape, position, anchor_offset);
        Ok(Mark1 { class_id: class_id, anchor_offset: anchor_offset, anchor: anchor })
    }
}

impl Walue<(u64, u16)> for Mark2 {
    fn read<T: Tape>(tape: &mut T, (position, class_count): (u64, u16)) -> Result<Self> {
        let anchor_offsets: Vec<u16> = try!(tape.take_given(class_count as usize));
        let anchors = jump_take!(@unwrap tape, position, class_count, anchor_offsets);
        Ok(Mark2 { anchor_offsets: anchor_offsets, anchors: anchors })
    }
}

impl Walue<u16> for Mark2Set {
    fn read<T: Tape>(tape: &mut T, class_count: u16) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            records.push(try!(tape.take_given((position, class_count))));
        }
        Ok(Mark2Set { count: count, records: records })
    }
}

impl Walue<u64> for Passage {
    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let entry_offset = try!(tape.take());
        let exit_offset = try!(tape.take());
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

impl Walue<(u64, SingleFlags, SingleFlags)> for Pair {
    #[inline]
    fn read<T: Tape>(tape: &mut T, (position, flags1, flags2): (u64, SingleFlags, SingleFlags))
                     -> Result<Self> {

        Ok(Pair {
            value1: try!(tape.take_given((position, flags1))),
            value2: try!(tape.take_given((position, flags2))),
        })
    }
}

impl Walue<(u64, SingleFlags, SingleFlags)> for PairSet {
    fn read<T: Tape>(tape: &mut T, parameter: (u64, SingleFlags, SingleFlags)) -> Result<Self> {
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(try!(tape.take_given(parameter)));
        }
        Ok(PairSet { count: count, records: records })
    }
}

impl Walue<(u64, SingleFlags)> for Single {
    fn read<T: Tape>(tape: &mut T, (position, flags): (u64, SingleFlags)) -> Result<Self> {
        macro_rules! take(
            ($flag:ident) => (if flags.$flag() { Some(try!(tape.take())) } else { None });
        );
        let x_placement = take!(has_x_placement);
        let y_placement = take!(has_y_placement);
        let x_advance = take!(has_x_advance);
        let y_advance = take!(has_y_advance);
        let device_x_placement_offset = take!(has_device_x_placement);
        let device_y_placement_offset = take!(has_device_y_placement);
        let device_x_advance_offset = take!(has_device_x_advance);
        let device_y_advance_offset = take!(has_device_y_advance);
        macro_rules! take(
            ($offset:ident) => (match $offset {
                Some(offset) => Some(jump_take!(@unwrap tape, position, offset)),
                _ => None,
            });
        );
        let device_x_placement = take!(device_x_placement_offset);
        let device_y_placement = take!(device_y_placement_offset);
        let device_x_advance = take!(device_x_advance_offset);
        let device_y_advance = take!(device_y_advance_offset);
        Ok(Single {
            x_placement: x_placement,
            y_placement: y_placement,
            x_advance: x_advance,
            y_advance: y_advance,
            device_x_placement_offset: device_x_placement_offset,
            device_y_placement_offset: device_y_placement_offset,
            device_x_advance_offset: device_x_advance_offset,
            device_y_advance_offset: device_y_advance_offset,
            device_x_placement: device_x_placement,
            device_y_placement: device_y_placement,
            device_x_advance: device_x_advance,
            device_y_advance: device_y_advance,
        })
    }
}
