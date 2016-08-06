//! The tables.

#![allow(unused_mut, unused_variables)]

use {Result, Tape, Walue};

mod adjustment;

pub use self::adjustment::{
    PairAdjustment,
    PairAdjustment1,
    PairAdjustment2,
    SingleAdjustment,
    SingleAdjustment1,
    SingleAdjustment2,
};

/// A table.
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
    #[doc = "A table for positioning glyphs in a chained context."]
    pub ChainedContextPositioning {
    }
}

table! {
    #[doc = "A table for positioning glyphs in a context."]
    pub ContextPositioning {
    }
}

table! {
    #[doc = "A table for attaching cursive glyphs."]
    pub CursiveAttachment {
    }
}

table! {
    #[doc = "A table for other types of positioning."]
    pub ExtensionPositioning {
    }
}

table! {
    #[doc = "A table for attaching combining marks to base glyphs."]
    pub MarkToBaseAttachment {
    }
}

table! {
    #[doc = "A table for attaching combining marks to ligatures."]
    pub MarkToLigatureAttachment {
    }
}

table! {
    #[doc = "A table for attaching combining marks to other marks."]
    pub MarkToMarkAttachment {
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleAdjustment(try!(tape.take())),
            2 => Table::PairAdjustment(try!(tape.take())),
            3 => Table::CursiveAttachment(try!(tape.take())),
            4 => Table::MarkToBaseAttachment(try!(tape.take())),
            5 => Table::MarkToLigatureAttachment(try!(tape.take())),
            6 => Table::MarkToMarkAttachment(try!(tape.take())),
            7 => Table::ContextPositioning(try!(tape.take())),
            8 => Table::ChainedContextPositioning(try!(tape.take())),
            9 => Table::ExtensionPositioning(try!(tape.take())),
            _ => raise!("found an unknown lookup type"),
        })
    }
}
