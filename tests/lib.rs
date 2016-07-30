extern crate opentype;

use opentype::{Value, q32};
use std::fs::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn glyph_positioning() {
    use opentype::GlyphPositioning;

    let table = ok!(GlyphPositioning::read(&mut setup(60412)));
    assert_eq!(table.header.version, q32(0x00010000));
    let tags = table.scripts.records.iter()
                                    .map(|record| record.tag.into())
                                    .collect::<Vec<[u8; 4]>>();
    assert_eq!(tags, &[*b"DFLT", *b"latn"]);
}

fn setup(seek: u64) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(seek)));
    file
}
