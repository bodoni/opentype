//! Lookup subtables.

#![allow(unused_mut, unused_variables)]

use {Result, Tape, Walue};

mod adjustment;

pub use self::adjustment::{
    ClassPair,
    ClassPairSet,
    PairAdjustment,
    PairAdjustment1,
    PairAdjustment2,
    SingleAdjustment,
    SingleAdjustment1,
    SingleAdjustment2,
};

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
