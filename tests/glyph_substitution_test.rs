extern crate opentype;
extern crate truetype;

use opentype::glyph_substitution::{GlyphSubstitution, SingleSubstitution, Table};
use opentype::layout::script::{Language, Script};
use truetype::Value;

#[macro_use]
mod common;

#[test]
fn features() {
    let GlyphSubstitution { features, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GSUB")));
    let tags = features
        .headers
        .iter()
        .map(|header| header.tag)
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert!(
        tags == tags![
            b"aalt", b"aalt", b"aalt", b"aalt", b"aalt",
            b"case", b"case", b"case", b"case", b"case",
            b"dnom", b"dnom", b"dnom", b"dnom", b"dnom",
            b"frac", b"frac", b"frac", b"frac", b"frac",
            b"liga", b"liga", b"liga", b"liga", b"liga",
            b"lnum", b"lnum", b"lnum", b"lnum", b"lnum",
            b"locl", b"locl", b"locl",
            b"numr", b"numr", b"numr", b"numr", b"numr",
            b"onum", b"onum", b"onum", b"onum", b"onum",
            b"ordn", b"ordn", b"ordn", b"ordn", b"ordn",
            b"pnum", b"pnum", b"pnum", b"pnum", b"pnum",
            b"sinf", b"sinf", b"sinf", b"sinf", b"sinf",
            b"subs", b"subs", b"subs", b"subs", b"subs",
            b"sups", b"sups", b"sups", b"sups", b"sups",
            b"tnum", b"tnum", b"tnum", b"tnum", b"tnum",
            b"zero", b"zero", b"zero", b"zero", b"zero",
        ]
    );
    let lookups = features
        .records
        .iter()
        .map(|record| record.lookup_count)
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert!(
        lookups == vec![
            2, 2, 2, 2, 2,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            3, 3, 3, 3, 3,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            3, 3, 3, 3, 3,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ]
    );
}

#[test]
fn lookups() {
    let GlyphSubstitution { lookups, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GSUB")));
    let kinds = lookups
        .records
        .iter()
        .map(|record| record.kind)
        .collect::<Vec<_>>();
    assert!(kinds == &[1, 3, 1, 1, 1, 1, 1, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1]);
    let record = &lookups.records[0];
    assert!(record.tables.len() == 1);
    match &record.tables[0] {
        &Table::SingleSubstitution(SingleSubstitution::Format2(ref table)) => {
            assert!(table.glyph_count == 61);
        }
        _ => unreachable!(),
    }
    let record = &lookups.records[17];
    assert!(record.tables.len() == 1);
    match &record.tables[0] {
        &Table::LigatureSubstitution(ref table) => {
            assert!(table.set_count == 1);
            let table = &table.sets[0];
            assert!(table.count == 3);
            let table = &table.records[0];
            assert!(table.component_count == 2);
        }
        _ => unreachable!(),
    }
}

#[test]
fn scripts() {
    let GlyphSubstitution { scripts, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GSUB")));
    let tags = scripts
        .headers
        .iter()
        .map(|header| header.tag)
        .collect::<Vec<_>>();
    assert!(tags == tags![b"DFLT", b"latn"]);
    assert!(scripts.get(Script::Default).is_some());
    assert!(scripts.get(Script::Latin).is_some());
    let tags = scripts
        .records
        .iter()
        .map(|record| {
            record
                .language_headers
                .iter()
                .map(|header| header.tag)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert!(tags == &[vec![], tags![b"AZE ", b"CRT ", b"TRK "]]);
    let record = &scripts.records[0];
    assert!(record.default_language.is_some());
    assert!(record.language_count == 0);
    let record = &scripts.records[1];
    assert!(record.language_count == 3);
    assert!(record.get(Language::Turkish).is_some());
}
