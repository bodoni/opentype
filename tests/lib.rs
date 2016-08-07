extern crate opentype;

use opentype::{Tag, Value};
use opentype::layout::Scripts;
use std::fs::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! tags(
    ($($name:expr),*) => (vec![$(Tag(*$name)),*]);
);

#[test]
fn glyph_positioning_features() {
    use opentype::GlyphPositioning;

    let GlyphPositioning { features, .. } = ok!(GlyphPositioning::read(&mut setup(60412)));
    let tags = features.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"kern", b"kern", b"kern", b"kern", b"kern",
                           b"size", b"size", b"size", b"size", b"size"]);
    let lookups = features.records.iter().map(|record| record.lookup_count).collect::<Vec<_>>();
    assert_eq!(lookups, &[1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);
}

#[test]
fn glyph_positioning_lookups() {
    use opentype::GlyphPositioning;
    use opentype::glyph_positioning::table::{PairAdjustment, Table};

    let GlyphPositioning { lookups, .. } = ok!(GlyphPositioning::read(&mut setup(60412)));
    assert_eq!(lookups.records.len(), 1);
    assert_eq!(lookups.records[0].kind, 2);
    assert!(lookups.records[0].mark_filtering_set.is_none());

    let tables = &lookups.records[0].tables;
    assert_eq!(tables.len(), 2);
    match &tables[0] {
        &Table::PairAdjustment(PairAdjustment::Format1(ref table)) => {
            assert_eq!(table.pair_set_count, 65);
        },
        _ => unreachable!(),
    }
    match &tables[1] {
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
    scripts(&ok!(GlyphPositioning::read(&mut setup(60412))).scripts);
}

#[test]
fn glyph_substitution_features() {
    use opentype::GlyphSubstitution;

    let GlyphSubstitution { features, .. } = ok!(GlyphSubstitution::read(&mut setup(57648)));
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
fn glyph_substitution_scripts() {
    use opentype::GlyphSubstitution;
    scripts(&ok!(GlyphSubstitution::read(&mut setup(57648))).scripts);
}

fn scripts(scripts: &Scripts) {
    let tags = scripts.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, tags![b"DFLT", b"latn"]);
    let tags = scripts.records.iter()
                              .map(|record| record.language_headers.iter()
                                                                   .map(|header| header.tag)
                                                                   .collect::<Vec<_>>())
                              .collect::<Vec<Vec<_>>>();
    assert_eq!(tags, &[vec![], tags![b"AZE ", b"CRT ", b"TRK "]]);
    assert!(scripts.records[0].default_language.is_some());
}

fn setup(seek: u64) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(seek)));
    file
}
