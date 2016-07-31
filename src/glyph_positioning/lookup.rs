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
        kind        (u16), // LookupType
        flags       (u16), // LookupFlag
        table_count (u16), // SubTableCount

        table_offsets (Vec<u16>) |tape, this| { // SubTable
            Walue::read(tape, this.table_count as usize)
        },

        mark_filtering_set (u16), // MarkFilteringSet
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
