//! The lookup list.

use {Result, Tape, Value};

mod class;
mod coverage;

pub use self::class::{Class, Class1, Class2, ClassRange};
pub use self::coverage::{Coverage, Coverage1, Coverage2, CoverageRange};

table! {
    @define
    #[doc = "A lookup list."]
    pub Lookups {
        count   (u16        ), // LookupCount
        offsets (Vec<u16>   ), // Lookup
        records (Vec<Record>),
    }
}

table! {
    @define
    #[doc = "A lookup record."]
    pub Record {
        kind               (u16        ), // LookupType
        flags              (Flags      ), // LookupFlag
        table_count        (u16        ), // SubTableCount
        table_offsets      (Vec<u16>   ), // SubTable
        mark_filtering_set (Option<u16>), // MarkFilteringSet
    }
}

flags! {
    #[doc = "Lookup flags."]
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
        let mut records: Vec<Record> = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + offsets[i] as u64));
            records.push(try!(tape.take()));
        }
        Ok(Lookups { count: count, offsets: offsets, records: records })
    }
}

impl Value for Record {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let kind = try!(tape.take());
        let flags = try!(tape.take::<Flags>());
        if flags.is_invalid() {
            raise!("found a malformed lookup record");
        }
        let table_count = try!(tape.take::<u16>());
        let table_offsets = try!(tape.take_given(table_count as usize));
        let mark_filtering_set = if flags.has_mark_filtering() {
            Some(try!(tape.take()))
        } else {
            None
        };
        Ok(Record {
            kind: kind,
            flags: flags,
            table_count: table_count,
            table_offsets: table_offsets,
            mark_filtering_set: mark_filtering_set,
        })
    }
}
