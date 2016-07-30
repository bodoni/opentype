extern crate opentype;

use opentype::Value;
use std::fs::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn glyph_positioning_scripts() {
    use opentype::GlyphPositioning;

    let table = ok!(GlyphPositioning::read(&mut setup(60412)));
    let scripts = &table.scripts;
    let tags = scripts.headers.iter().map(|header| header.tag.into()).collect::<Vec<[u8; 4]>>();
    assert_eq!(tags, &[*b"DFLT", *b"latn"]);
    let tags = scripts.records.iter()
                              .map(|record| record.language_headers.iter()
                                                                   .map(|header| header.tag.into())
                                                                   .collect::<Vec<[u8; 4]>>())
                                        .collect::<Vec<Vec<_>>>();
    assert_eq!(tags, &[vec![], vec![*b"AZE ", *b"CRT ", *b"TRK "]]);
    assert!(scripts.records[0].default_language.is_some());
}

fn setup(seek: u64) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(seek)));
    file
}
