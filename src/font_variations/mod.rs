//! The [font variations][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/fvar

use truetype::naming_table::NameID;
use truetype::{q32, Result, Tag, Tape, Value, Walue};

table! {
    @define
    #[doc = "Font variations."]
    pub FontVariations {
        header           (Header             ),
        axis_records     (Vec<AxisRecord>    ),
        instance_records (Vec<InstanceRecord>),
    }
}

table! {
    #[doc = "The header of font variations."]
    #[derive(Copy)]
    pub Header {
        major_version  (u16), // majorVersion
        minor_version  (u16), // minorVersion
        axis_offset    (u16), // axesArrayOffset
        reserved       (u16) = { 2 }, // reserved
        axis_count     (u16), // axisCount
        axis_size      (u16), // axisSize
        instance_count (u16), // instanceCount
        instance_size  (u16), // instanceSize
    }
}

table! {
    #[doc = "An axis record of font variations."]
    #[derive(Copy)]
    pub AxisRecord { // VariationAxisRecord
        tag           (Tag      ), // tag
        min_value     (q32      ), // minValue
        default_value (q32      ), // defaultValue
        max_value     (q32      ), // maxValue
        flags         (AxisFlags), // flags
        name_id       (NameID   ), // axisNameID
    }
}

table! {
    @define
    #[doc = "An instance record of font variations."]
    pub InstanceRecord { // InstanceRecord
        subfamily_name_id  (NameID       ), // subfamilyNameID
        flags              (InstanceFlags), // flags
        coordinates        (Vec<q32>     ), // coordinates
        postscript_name_id (NameID       ), // postScriptNameID
    }
}

flags! {
    #[doc = "Axis flags."]
    pub AxisFlags(u16) {
        0b0000_0000_0000_0001 => is_hidden,
        0b1111_1111_1111_1110 => is_invalid,
    }
}

flags! {
    #[doc = "Instance flags."]
    pub InstanceFlags(u16) {
        0b1111_1111_1111_1111 => is_invalid,
    }
}

impl Value for FontVariations {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let mut position = tape.position()?;
        let header: Header = tape.take()?;
        position += header.axis_offset as u64;
        let axis_records = (0..header.axis_count as u64)
            .map(|i| {
                tape.jump(position + i * header.axis_size as u64)?;
                tape.take()
            })
            .collect::<Result<Vec<_>>>()?;
        position += header.axis_count as u64 * header.axis_size as u64;
        let instance_records = (0..header.instance_count as u64)
            .map(|i| {
                tape.jump(position + i * header.instance_size as u64)?;
                tape.take_given(header.axis_count)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            header,
            axis_records,
            instance_records,
        })
    }
}

impl Walue<'static> for InstanceRecord {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, axis_count: Self::Parameter) -> Result<Self> {
        Ok(Self {
            subfamily_name_id: tape.take()?,
            flags: tape.take()?,
            coordinates: tape.take_given(axis_count as usize)?,
            postscript_name_id: tape.take()?,
        })
    }
}
