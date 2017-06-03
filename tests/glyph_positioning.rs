use opentype::glyph_positioning::{GlyphPositioning, PairAdjustment, Table};
use opentype::layout::script::{Language, Script};
use truetype::Value;

#[test]
fn features() {
    let GlyphPositioning { features, .. } = ok!(Value::read(&mut setup!(CFF, "GPOS")));
    let tags = features.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags,
               tags![b"kern", b"kern", b"kern", b"kern", b"kern", b"size", b"size", b"size",
                     b"size", b"size"]);
    let lookups = features.records.iter().map(|record| record.lookup_count).collect::<Vec<_>>();
    assert_eq!(lookups, &[1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);
}

#[test]
fn lookups() {
    let GlyphPositioning { lookups, .. } = ok!(Value::read(&mut setup!(CFF, "GPOS")));
    assert_eq!(lookups.records.len(), 1);
    let record = &lookups.records[0];
    assert!(record.mark_filtering_set.is_none());
    assert_eq!(record.tables.len(), 2);
    match &record.tables[0] {
        &Table::PairAdjustment(PairAdjustment::Format1(ref table)) => {
            assert_eq!(table.set_count, 65);
        },
        _ => unreachable!(),
    }
    match &record.tables[1] {
        &Table::PairAdjustment(PairAdjustment::Format2(ref table)) => {
            assert_eq!(table.class1_count, 99);
            assert_eq!(table.class2_count, 95);
        },
        _ => unreachable!(),
    }
}

#[test]
fn scripts() {
    let GlyphPositioning { scripts, .. } = ok!(Value::read(&mut setup!(CFF, "GPOS")));
    let tags = scripts.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"DFLT", b"latn"]);
    assert!(scripts.get(Script::Default).is_some());
    assert!(scripts.get(Script::Latin).is_some());
    let tags = scripts
        .records
        .iter()
        .map(|record| record.language_headers.iter().map(|header| header.tag).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(tags, &[vec![], tags![b"AZE ", b"CRT ", b"TRK "]]);
    let record = &scripts.records[0];
    assert!(record.default_language.is_some());
    assert_eq!(record.language_count, 0);
    let record = &scripts.records[1];
    assert_eq!(record.language_count, 3);
    assert!(record.get(Language::Turkish).is_some());
}
