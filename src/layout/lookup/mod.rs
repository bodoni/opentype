//! The lookup list.

use truetype::GlyphID;

use {Result, Tape, Value};

mod class;
mod coverage;

pub mod table;

pub use self::class::{Class, Class1, Class2};
pub use self::coverage::{Coverage, Coverage1, Coverage2};
pub use self::table::Table;

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
        kind               (Kind       ), // LookupType
        flags              (Flags      ), // LookupFlag
        table_count        (u16        ), // SubTableCount
        table_offsets      (Vec<u16>   ), // SubTable
        mark_filtering_set (Option<u16>), // MarkFilteringSet
        table_records      (Vec<Table> ),
    }
}

/// A lookup type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    SingleAdjustment = 1,
    PairAdjustment = 2,
    CursiveAttachment = 3,
    MarkToBaseAttachment = 4,
    MarkToLigatureAttachment = 5,
    MarkToMarkAttachment = 6,
    ContextPositioning = 7,
    ChainedContextPositioning = 8,
    ExtensionPositioning = 9,
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

table! {
    #[doc = "A glyph range."]
    #[derive(Copy)]
    pub Range {
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // Class or StartCoverageIndex
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
        let position = try!(tape.position());
        let kind = try!(tape.take::<Kind>());
        let flags = try!(tape.take::<Flags>());
        if flags.is_invalid() {
            raise!("found a malformed lookup record");
        }
        let table_count = try!(tape.take::<u16>());
        let table_offsets: Vec<u16> = try!(tape.take_given(table_count as usize));
        let mark_filtering_set = if flags.has_mark_filtering() {
            Some(try!(tape.take()))
        } else {
            None
        };
        let mut table_records = Vec::with_capacity(table_count as usize);
        for i in 0..(table_count as usize) {
            try!(tape.jump(position + table_offsets[i] as u64));
            table_records.push(try!(tape.take_given(kind)));
        }
        Ok(Record {
            kind: kind,
            flags: flags,
            table_count: table_count,
            table_offsets: table_offsets,
            mark_filtering_set: mark_filtering_set,
            table_records: table_records,
        })
    }
}

impl Value for Kind {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let kind = try!(tape.take::<u16>());
        if kind < 1 || kind > 9 {
            raise!("found an unknown lookup type");
        }
        Ok(unsafe { ::std::mem::transmute(kind as u8) })
    }
}
