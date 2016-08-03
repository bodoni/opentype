//! The subtables.

#![allow(unused_mut, unused_variables)]

use {Result, Tape, Value, Walue};

use super::{Coverage, Kind};

/// A lookup subtable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Table {
    SingleAdjustment(SingleAdjustment),
    PairAdjustment(PairAdjustment),
    CursiveAttachment(CursiveAttachment),
    MarkToBaseAttachment(MarkToBaseAttachment),
    MarkToLigatureAttachment(MarkToLigatureAttachment),
    MarkToMarkAttachment(MarkToMarkAttachment),
    ContextPositioning(ContextPositioning),
    ChainedContextPositioning(ChainedContextPositioning),
    ExtensionPositioning(ExtensionPositioning),
}

table! {
    #[doc = "A positioning of one or more glyphs in a chained context."]
    pub ChainedContextPositioning {
    }
}

table! {
    #[doc = "A positioning of one or more glyphs in a context."]
    pub ContextPositioning {
    }
}

table! {
    #[doc = "An attachment of cursive glyphs."]
    pub CursiveAttachment {
    }
}

table! {
    #[doc = "An extension mechanism for other positionings."]
    pub ExtensionPositioning {
    }
}

table! {
    #[doc = "An attachment of a combining mark to a base glyph."]
    pub MarkToBaseAttachment {
    }
}

table! {
    #[doc = "An attachment of a combining mark to a ligature."]
    pub MarkToLigatureAttachment {
    }
}

table! {
    #[doc = "An attachment of a combining mark to another mark."]
    pub MarkToMarkAttachment {
    }
}

table! {
    #[doc = "A position adjustment of a pair of glyphs."]
    pub PairAdjustment {
    }
}

/// A position adjustment of a single glyph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleAdjustment {
    /// Format 1.
    Format1(SingleAdjustment1),
    /// Format 2.
    Format2(SingleAdjustment2),
}

table! {
    @define
    #[doc = "A position adjustment of a single glyph in format 1."]
    pub SingleAdjustment1 {
        format          (u16     ), // PosFormat
        coverage_offset (u16     ), // Coverage
        value_flags     (Flags   ), // ValueFormat
        value           (Record  ), // Value
        coverage        (Coverage),
    }
}

table! {
    @define
    #[doc = "A position adjustment of a single glyph in format 2."]
    pub SingleAdjustment2 {
        format          (u16        ), // PosFormat
        coverage_offset (u16        ), // Coverage
        value_flags     (Flags      ), // ValueFormat
        value_count     (u16        ), // ValueCount
        values          (Vec<Record>), // Value
        coverage        (Coverage   ),
    }
}

flags! {
    #[doc = "Value-record flags."]
    pub Flags(u16) {
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
    #[doc = "A value record."]
    pub Record { // Value
        x_placement               (Option<i16>), // XPlacement
        y_placement               (Option<i16>), // YPlacement
        x_advance                 (Option<i16>), // XAdvance
        y_advance                 (Option<i16>), // YAdvance
        device_x_placement_offset (Option<u16>), // XPlaDevice
        device_y_placement_offset (Option<u16>), // YPlaDevice
        device_x_advance_offset   (Option<u16>), // XAdvDevice
        device_y_advance_offset   (Option<u16>), // YAdvDevice
    }
}

impl Walue<Kind> for Table {
    fn read<T: Tape>(tape: &mut T, kind: Kind) -> Result<Self> {
        Ok(match kind {
            Kind::ChainedContextPositioning => {
                Table::ChainedContextPositioning(try!(tape.take()))
            },
            Kind::ContextPositioning => {
                Table::ContextPositioning(try!(tape.take()))
            },
            Kind::CursiveAttachment => {
                Table::CursiveAttachment(try!(tape.take()))
            },
            Kind::ExtensionPositioning => {
                Table::ExtensionPositioning(try!(tape.take()))
            },
            Kind::MarkToBaseAttachment => {
                Table::MarkToBaseAttachment(try!(tape.take()))
            },
            Kind::MarkToLigatureAttachment => {
                Table::MarkToLigatureAttachment(try!(tape.take()))
            },
            Kind::MarkToMarkAttachment => {
                Table::MarkToMarkAttachment(try!(tape.take()))
            },
            Kind::PairAdjustment => {
                Table::PairAdjustment(try!(tape.take()))
            },
            Kind::SingleAdjustment => {
                Table::SingleAdjustment(try!(tape.take()))
            },
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleAdjustment::Format1(try!(tape.take())),
            2 => SingleAdjustment::Format2(try!(tape.take())),
            _ => raise!("found an unknow format of the single-adjustment subtable"),
        })
    }
}

impl Value for SingleAdjustment1 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value_flags: Flags = try!(tape.take());
        if value_flags.is_invalid() {
            raise!("found a malformed value record");
        }
        let value = try!(tape.take_given(value_flags));
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleAdjustment1 {
            format: format,
            coverage_offset: coverage_offset,
            value_flags: value_flags,
            value: value,
            coverage: coverage,
        })
    }
}

impl Value for SingleAdjustment2 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value_flags: Flags = try!(tape.take());
        if value_flags.is_invalid() {
            raise!("found a malformed value record");
        }
        let value_count = try!(tape.take());
        let mut values = Vec::with_capacity(value_count as usize);
        for i in 0..(value_count as usize) {
            values.push(try!(tape.take_given(value_flags)));
        }
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleAdjustment2 {
            format: format,
            coverage_offset: coverage_offset,
            value_flags: value_flags,
            value_count: value_count,
            values: values,
            coverage: coverage,
        })
    }
}

impl Walue<Flags> for Record {
    fn read<T: Tape>(tape: &mut T, flags: Flags) -> Result<Self> {
        macro_rules! read(
            ($flag:ident) => (if flags.$flag() { Some(try!(tape.take())) } else { None });
        );
        Ok(Record {
            x_placement: read!(has_x_placement),
            y_placement: read!(has_y_placement),
            x_advance: read!(has_x_advance),
            y_advance: read!(has_y_advance),
            device_x_placement_offset: read!(has_device_x_placement),
            device_y_placement_offset: read!(has_device_y_placement),
            device_x_advance_offset: read!(has_device_x_advance),
            device_y_advance_offset: read!(has_device_y_advance),
        })
    }
}
