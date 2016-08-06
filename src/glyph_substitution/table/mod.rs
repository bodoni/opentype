//! The tables.

#![allow(unused_mut, unused_variables)]

use {Result, Tape, Walue};

/// A table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Table {
    SingleSubstibution(SingleSubstibution),
    MultipleSubstibution(MultipleSubstibution),
    AlternateSubstibution(AlternateSubstibution),
    LigatureSubstibution(LigatureSubstibution),
    ContextSubstibution(ContextSubstibution),
    ChainedContextSubstibution(ChainedContextSubstibution),
    ExtensionSubstibution(ExtensionSubstibution),
    ReverseChainedContextSubstibution(ReverseChainedContextSubstibution),
}

table! {
    #[doc = "A table for substituting one glyph with one of many glyphs."]
    pub AlternateSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in a chained context."]
    pub ChainedContextSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in a context."]
    pub ContextSubstibution {
    }
}

table! {
    #[doc = "A table for other types of substitution."]
    pub ExtensionSubstibution {
    }
}

table! {
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub LigatureSubstibution {
    }
}

table! {
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub MultipleSubstibution {
    }
}

table! {
    #[doc = "A table for substituting one glyph with one glyph."]
    pub SingleSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in reverse order in a chained context."]
    pub ReverseChainedContextSubstibution {
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleSubstibution(try!(tape.take())),
            2 => Table::MultipleSubstibution(try!(tape.take())),
            3 => Table::AlternateSubstibution(try!(tape.take())),
            4 => Table::LigatureSubstibution(try!(tape.take())),
            5 => Table::ContextSubstibution(try!(tape.take())),
            6 => Table::ChainedContextSubstibution(try!(tape.take())),
            7 => Table::ExtensionSubstibution(try!(tape.take())),
            8 => Table::ReverseChainedContextSubstibution(try!(tape.take())),
            _ => raise!("found an unknown lookup type"),
        })
    }
}
