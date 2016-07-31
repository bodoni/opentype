//! The lookups.

use {Result, Tape, Value, Walue};

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
    #[doc = "A lookup."]
    pub Lookup {
        kind (u16), // LookupType

        flags (Flags) |tape, this| { // LookupFlag
            let value = try!(tape.take::<Flags>());
            if value.is_invalid() {
                raise!("found a malformed lookup");
            }
            Ok(value)
        },

        table_count (u16), // SubTableCount

        table_offsets (Vec<u16>) |tape, this| { // SubTable
            Walue::read(tape, this.table_count as usize)
        },

        mark_filtering_set (u16), // MarkFilteringSet
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
