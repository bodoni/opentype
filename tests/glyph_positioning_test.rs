extern crate opentype;
extern crate truetype;

use opentype::glyph_positioning::{GlyphPositioning, PairAdjustment, Table};
use opentype::layout::script::{Language, Script};
use truetype::Value;

#[macro_use]
mod common;

#[test]
fn features() {
    let GlyphPositioning { features, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GPOS")));
    let tags = features
        .headers
        .iter()
        .map(|header| header.tag)
        .collect::<Vec<_>>();
    assert!(
        tags
            == tags![
                b"kern", b"kern", b"kern", b"kern", b"kern",
                b"size", b"size", b"size", b"size", b"size",
            ]
    );
    let lookups = features
        .records
        .iter()
        .map(|record| record.lookup_count)
        .collect::<Vec<_>>();
    assert!(lookups == &[1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);
}

#[test]
fn lookups() {
    let GlyphPositioning { lookups, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GPOS")));
    assert!(lookups.records.len() == 1);
    let record = &lookups.records[0];
    assert!(record.mark_filtering_set.is_none());
    assert!(record.tables.len() == 2);
    match &record.tables[0] {
        &Table::PairAdjustment(PairAdjustment::Format1(ref table)) => {
            assert!(table.set_count == 65);
        }
        _ => unreachable!(),
    }
    match &record.tables[1] {
        &Table::PairAdjustment(PairAdjustment::Format2(ref table)) => {
            assert!(table.class1_count == 99);
            assert!(table.class2_count == 95);
        }
        _ => unreachable!(),
    }
}

#[test]
fn scripts() {
    let GlyphPositioning { scripts, .. } = ok!(Value::read(&mut setup!(SourceSerifPro, "GPOS")));
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
