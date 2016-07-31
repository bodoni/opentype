//! The lookups.

use truetype::GlyphID;

use {Result, Tape, Value};

table! {
    @define
    #[doc = "A lookup list."]
    pub Lookups {
        count   (u16        ), // LookupCount
        offsets (Vec<u16>   ), // Lookup
        records (Vec<Lookup>),
    }
}

table! {
    @define
    #[doc = "A lookup."]
    pub Lookup {
        kind               (u16     ), // LookupType
        flags              (Flags   ), // LookupFlag
        table_count        (u16     ), // SubTableCount
        table_offsets      (Vec<u16>), // SubTable
        mark_filtering_set (u16     ), // MarkFilteringSet
    }
}

/// A coverage table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Coverage {
    /// A coverage table of format 1.
    Format1(Coverage1),
    /// A coverage table of format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage table of format 1."]
    pub Coverage1 {
        format (u16), // CoverageFormat
        count  (u16), // GlyphCount

        glyphs (Vec<GlyphID>) |tape, this| { // GlyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage table of format 2."]
    pub Coverage2 {
        format (u16), // CoverageFormat
        count  (u16), // RangeCount

        ranges (Vec<Range>) |tape, this| { // RangeRecord
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A glyph range."]
    #[derive(Copy)]
    pub Range {
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // StartCoverageIndex
    }
}

flags! {
    #[doc = "Flags of a lookup."]
    pub Flags(u16) {
        0b0000_0000_0000_0001 => is_right_to_left,
        0b0000_0000_0000_0010 => should_ignore_base_glyphs,
        0b0000_0000_0000_0100 => should_ignore_ligature,
        0b0000_0000_0000_1000 => should_ignore_marks,
        0b0000_0000_0001_0000 => has_mark_filtering,
        0b0000_0000_1110_0000 => is_invalid,
    }
}

impl Value for Lookups {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take::<u16>());
        let offsets: Vec<u16> = try!(tape.take_given(count as usize));
        let mut records: Vec<Lookup> = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + offsets[i] as u64));
            records.push(try!(tape.take()));
        }
        Ok(Lookups { count: count, offsets: offsets, records: records })
    }
}

impl Value for Lookup {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let kind = try!(tape.take::<u16>());
        let flags = try!(tape.take::<Flags>());
        if flags.is_invalid() {
            raise!("found a malformed lookup");
        }
        let table_count = try!(tape.take::<u16>());
        let table_offsets: Vec<u16> = try!(tape.take_given(table_count as usize));
        let mark_filtering_set = try!(tape.take());
        Ok(Lookup {
            kind: kind,
            flags: flags,
            table_count: table_count,
            table_offsets: table_offsets,
            mark_filtering_set: mark_filtering_set,
        })
    }
}

impl Value for Coverage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Coverage::Format1(try!(tape.take())),
            2 => Coverage::Format2(try!(tape.take())),
            _ => raise!("found a coverage table of an unsupported format"),
        })
    }
}
