extern crate opentype;

use opentype::Font;
use opentype::compound::{CharMapping, MaxProfile};
use std::fs::{self, File};
use std::path::PathBuf;

#[test]
fn char_mapping_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();

    let header = font.char_mapping_header.as_ref().unwrap();
    assert_eq!(header.version, 0);
    assert_eq!(header.numTables, 3);
}

#[test]
fn char_mappings() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();

    assert_eq!(font.char_mappings.len(), 3);
    match &font.char_mappings[0] {
        &CharMapping::Format4(ref mapping) => {
            assert_eq!(mapping.segCountX2, 2 * 115);
            assert_eq!(mapping.searchRange, 2 * (1 << 115f64.log2().floor() as usize));
            assert_eq!(mapping.endCount.len(), 115);
        },
        _ => unreachable!(),
    }
    match &font.char_mappings[1] {
        &CharMapping::Format6(..) => {},
        _ => unreachable!(),
    }
    match &font.char_mappings[2] {
        &CharMapping::Format4(..) => {},
        _ => unreachable!(),
    }

    match font.max_profile {
        Some(MaxProfile::Version05(ref profile)) => {
            assert_eq!(profile.numGlyphs, 545);
        },
        _ => unreachable!(),
    }
}

#[test]
fn encoding_records() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();

    let records = &font.encoding_records;
    assert_eq!(records.len(), 3);

    let (platforms, encodings) = ([0, 1, 3], [3, 0, 1]);
    for i in 0..3 {
        assert_eq!(records[i].platformID, platforms[i]);
        assert_eq!(records[i].encodingID, encodings[i]);
    }
}

#[test]
fn font_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();

    match font.font_header {
        Some(ref header) => {
            assert_eq!(header.fontRevision.as_f32(), 1.014);
            assert_eq!(header.unitsPerEm, 1000);
            assert_eq!(header.macStyle, 0);
        },
        _ => unreachable!(),
    }
}

#[test]
fn offset_table() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = &font.offset_table;

    assert_eq!(table.version.0, 0x4f54544f);
    assert_eq!(table.numTables, 12);
    assert_eq!(table.searchRange, 8 * 16);
    assert_eq!(table.entrySelector, 3);
    assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
}

fn open(name: &str) -> File {
    let path = PathBuf::from("tests/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    File::open(&path).unwrap()
}
