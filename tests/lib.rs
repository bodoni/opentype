extern crate opentype;

use opentype::Font;
use opentype::table::{CharMappingEncoding, MaximumProfile};
use std::fs::{self, File};
use std::path::PathBuf;

mod fixture;

#[test]
fn char_mapping_encodings() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let encodings = &font.char_mapping.encodings;

    assert_eq!(encodings.len(), 3);
    match &encodings[0] {
        &CharMappingEncoding::Format4(ref encoding) => {
            assert_eq!(encoding.segCountX2, 2 * 115);
            assert_eq!(encoding.searchRange, 2 * (1 << 115f64.log2().floor() as usize));
            assert_eq!(encoding.endCode.len(), 115);
            assert_eq!(encoding.startCode.len(), 115);
            assert_eq!(encoding.idDelta.len(), 115);
            assert_eq!(encoding.idRangeOffset.len(), 115);
            assert_eq!(encoding.glyphIdArray.len(), 251);
            assert_eq!(encoding.mapping(), fixture::mapping());
        },
        _ => unreachable!(),
    }
    match &encodings[1] {
        &CharMappingEncoding::Format6(ref encoding) => {
            assert_eq!(encoding.firstCode, 9);
            assert_eq!(encoding.entryCount, 247);
            assert_eq!(encoding.glyphIdArray.len(), 247);
        },
        _ => unreachable!(),
    }
    match &encodings[2] {
        &CharMappingEncoding::Format4(ref encoding) => {
            assert_eq!(encoding.segCountX2, 2 * 115);
        },
        _ => unreachable!(),
    }
}

#[test]
fn char_mapping_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let header = &font.char_mapping.header;

    assert_eq!(header.version, 0);
    assert_eq!(header.numTables, 3);
}

#[test]
fn char_mapping_records() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let records = &font.char_mapping.records;

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
    let header = &font.font_header;

    assert_eq!(header.fontRevision.as_f32(), 1.014);
    assert_eq!(header.unitsPerEm, 1000);
    assert_eq!(header.macStyle, 0);
}

#[test]
fn horizontal_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let header = &font.horizontal_header;

    assert_eq!(header.Ascender, 918);
    assert_eq!(header.Descender, -335);
    assert_eq!(header.numberOfHMetrics, 521);
}

#[test]
fn offset_table_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let header = &font.offset_table.header;

    assert_eq!(header.version.0, 0x4f54544f);
    assert_eq!(header.numTables, 12);
    assert_eq!(header.searchRange, 8 * 16);
    assert_eq!(header.entrySelector, 3);
    assert_eq!(header.rangeShift, header.numTables * 16 - header.searchRange);
}

#[test]
fn maximum_profile() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let profile = &font.maximum_profile;

    match profile {
        &MaximumProfile::Version05(ref profile) => {
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
