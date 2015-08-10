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
    let mappings = &font.char_mappings;

    assert_eq!(mappings.len(), 3);
    match &mappings[0] {
        &CharMapping::Format4(ref mapping) => {
            assert_eq!(mapping.segCountX2, 2 * 115);
            assert_eq!(mapping.searchRange, 2 * (1 << 115f64.log2().floor() as usize));
            assert_eq!(mapping.endCount.len(), 115);
            assert_eq!(mapping.startCount.len(), 115);
            assert_eq!(mapping.idDelta.len(), 115);
            assert_eq!(mapping.idRangeOffset.len(), 115);
        },
        _ => unreachable!(),
    }
    match &mappings[1] {
        &CharMapping::Format6(..) => {},
        _ => unreachable!(),
    }
    match &mappings[2] {
        &CharMapping::Format4(..) => {},
        _ => unreachable!(),
    }
}

#[test]
fn encoding_records() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let records = &font.encoding_records;

    assert_eq!(records.len(), 3);
    assert_eq!(records[0].platformID, 0);
    assert_eq!(records[0].encodingID, 3);
    assert_eq!(records[1].platformID, 1);
    assert_eq!(records[1].encodingID, 0);
    assert_eq!(records[2].platformID, 3);
    assert_eq!(records[2].encodingID, 1);
}

#[test]
fn font_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let header = font.font_header.as_ref().unwrap();

    assert_eq!(header.fontRevision.as_f32(), 1.014);
    assert_eq!(header.unitsPerEm, 1000);
    assert_eq!(header.macStyle, 0);
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

#[test]
fn max_profile() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let profile = font.max_profile.as_ref().unwrap();

    match profile {
        &MaxProfile::Version05(ref profile) => {
            assert_eq!(profile.numGlyphs, 545);
        },
        _ => unreachable!(),
    }
}

fn open(name: &str) -> File {
    let path = PathBuf::from("tests/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    File::open(&path).unwrap()
}
