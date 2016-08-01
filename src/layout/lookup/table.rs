//! The subtables.

#![allow(unused_mut, unused_variables)]

use {Result, Tape, Walue};

use super::Kind;

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

table! {
    #[doc = "A position adjustment of a single glyph."]
    pub SingleAdjustment {
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
