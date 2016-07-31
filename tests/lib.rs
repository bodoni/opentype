extern crate opentype;

use opentype::{Tag, Value};
use std::fs::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn glyph_positioning_features() {
    use opentype::GlyphPositioning;

    let GlyphPositioning { features, .. } = ok!(GlyphPositioning::read(&mut setup(60412)));
    let tags = features.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, &[Tag(*b"kern"), Tag(*b"kern"), Tag(*b"kern"), Tag(*b"kern"), Tag(*b"kern"),
                       Tag(*b"size"), Tag(*b"size"), Tag(*b"size"), Tag(*b"size"), Tag(*b"size")]);
}

#[test]
fn glyph_positioning_scripts() {
    use opentype::GlyphPositioning;

    let GlyphPositioning { scripts, .. } = ok!(GlyphPositioning::read(&mut setup(60412)));
    let tags = scripts.headers.iter().map(|header| header.tag).collect::<Vec<_>>();
    assert_eq!(tags, &[Tag(*b"DFLT"), Tag(*b"latn")]);
    let tags = scripts.records.iter()
                              .map(|record| record.language_headers.iter()
                                                                   .map(|header| header.tag)
                                                                   .collect::<Vec<_>>())
                              .collect::<Vec<Vec<_>>>();
    assert_eq!(tags, &[vec![], vec![Tag(*b"AZE "), Tag(*b"CRT "), Tag(*b"TRK ")]]);
    assert!(scripts.records[0].default_language.is_some());
}

fn setup(seek: u64) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(seek)));
    file
}
