extern crate opentype;
extern crate truetype;

use opentype::glyph_transformation::Scripts;
use std::fs::File;
use truetype::{Tag, Value};

const GPOS: u64 = 60412;
const GSUB: u64 = 57648;

macro_rules! ok(($result:expr) => ($result.unwrap()));
macro_rules! tags(($($name:expr),*) => (vec![$(Tag(*$name)),*]));

#[test]
fn glyph_positioning_features() {
    use opentype::GlyphPositioning;

    let GlyphPositioning { features, .. } = ok!(Value::read(&mut setup(GPOS)));
    let tags = features.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"kern", b"kern", b"kern", b"kern", b"kern",
                           b"size", b"size", b"size", b"size", b"size"]);
    let lookups = features.records.iter().map(|record| record.lookup_count).collect::<Vec<_>>();
    assert_eq!(lookups, &[1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);
}

#[test]
fn glyph_positioning_lookups() {
    use opentype::GlyphPositioning;
    use opentype::glyph_positioning::{PairAdjustment, Table};

    let GlyphPositioning { lookups, .. } = ok!(Value::read(&mut setup(GPOS)));
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
fn glyph_positioning_scripts() {
    use opentype::GlyphPositioning;
    scripts(&ok!(GlyphPositioning::read(&mut setup(GPOS))).scripts);
}

#[test]
fn glyph_substitution_features() {
    use opentype::GlyphSubstitution;

    let GlyphSubstitution { features, .. } = ok!(Value::read(&mut setup(GSUB)));
    let tags = features.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"aalt", b"aalt", b"aalt", b"aalt", b"aalt",
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
                           b"zero", b"zero", b"zero", b"zero", b"zero"]);
    let lookups = features.records.iter().map(|record| record.lookup_count).collect::<Vec<_>>();
    assert_eq!(lookups, vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 1, 1, 1,
                             1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                             1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 1,
                             1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn glyph_substitution_lookups() {
    use opentype::GlyphSubstitution;
    use opentype::glyph_substitution::{SingleSubstitution, Table};

    let GlyphSubstitution { lookups, .. } = ok!(Value::read(&mut setup(GSUB)));
    let kinds = lookups.records.iter().map(|record| record.kind).collect::<Vec<_>>();
    assert_eq!(kinds, &[1, 3, 1, 1, 1, 1, 1, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1]);
    let record = &lookups.records[0];
    assert_eq!(record.tables.len(), 1);
    match &record.tables[0] {
        &Table::SingleSubstitution(SingleSubstitution::Format2(ref table)) => {
            assert_eq!(table.glyph_count, 61);
        },
        _ => unreachable!(),
    }
    let record = &lookups.records[17];
    assert_eq!(record.tables.len(), 1);
    match &record.tables[0] {
        &Table::LigatureSubstitution(ref table) => {
            assert_eq!(table.set_count, 1);
            let table = &table.sets[0];
            assert_eq!(table.count, 3);
            let table = &table.records[0];
            assert_eq!(table.component_count, 2);
        },
        _ => unreachable!(),
    }
}

#[test]
fn glyph_substitution_scripts() {
    use opentype::GlyphSubstitution;
    scripts(&ok!(GlyphSubstitution::read(&mut setup(GSUB))).scripts);
}

fn scripts(scripts: &Scripts) {
    use opentype::glyph_transformation::script::{Language, Script};

    let tags = scripts.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"DFLT", b"latn"]);
    assert!(scripts.get(Script::Default).is_some());
    assert!(scripts.get(Script::Latin).is_some());
    let tags = scripts.records.iter()
                              .map(|record| record.language_headers.iter()
                                                                   .map(|header| header.tag)
                                                                   .collect::<Vec<_>>())
                              .collect::<Vec<_>>();
    assert_eq!(tags, &[vec![], tags![b"AZE ", b"CRT ", b"TRK "]]);
    let record = &scripts.records[0];
    assert!(record.default_language.is_some());
    assert_eq!(record.language_count, 0);
    let record = &scripts.records[1];
    assert_eq!(record.language_count, 3);
    assert!(record.get(Language::Turkish).is_some());
}

fn setup(seek: u64) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(seek)));
    file
}
