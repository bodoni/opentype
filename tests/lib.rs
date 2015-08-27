extern crate opentype;

use opentype::Font;
use std::fs::{self, File};
use std::path::PathBuf;

#[test]
fn font_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = &font.font_header.as_ref().unwrap();

    assert_eq!(format!("{:.3}", table.fontRevision.as_f32()), "1.017");
    assert_eq!(table.unitsPerEm, 1000);
    assert_eq!(table.macStyle, 0);
}

#[test]
fn horizontal_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.horizontal_header.as_ref().unwrap();

    assert_eq!(table.Ascender, 918);
    assert_eq!(table.Descender, -335);
    assert_eq!(table.numberOfHMetrics, 547);
}

#[test]
fn horizontal_metrics() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.horizontal_metrics.as_ref().unwrap();

    assert_eq!(table.hMetrics.len(), 547);
    assert_eq!(table.leftSideBearing.len(), 547 - 547);
}

#[test]
fn offset_table_header() {
    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = &font.offset_table.header;

    assert_eq!(table.version.0, 0x4f54544f);
    assert_eq!(table.numTables, 12);
    assert_eq!(table.searchRange, 8 * 16);
    assert_eq!(table.entrySelector, 3);
    assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
}

#[test]
fn maximum_profile() {
    use opentype::table::MaximumProfile;

    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.maximum_profile.as_ref().unwrap();

    match table {
        &MaximumProfile::Version05(ref table) => {
            assert_eq!(table.numGlyphs, 547);
        },
        _ => unreachable!(),
    }
}

#[test]
fn naming_table() {
    use opentype::table::NamingTable;

    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.naming_table.as_ref().unwrap();

    match table {
        &NamingTable::Format0(ref table) => {
            assert_eq!(table.count, 26);
            assert_eq!(table.strings().unwrap()[9], "Frank Grießhammer");
        },
        _ => unreachable!(),
    }
}

#[test]
fn postscript() {
    use opentype::table::PostScript;

    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.postscript.as_ref().unwrap();

    match table {
        &PostScript::Version30(ref table) => {
            assert_eq!(table.version.as_f32(), 3.0);
            assert_eq!(table.underlinePosition, -75);
        },
        _ => unreachable!(),
    }
}

#[test]
fn windows_metrics() {
    use opentype::table::WindowsMetrics;

    let mut file = open("SourceSerifPro-Regular.otf");
    let font = Font::read(&mut file).unwrap();
    let table = font.windows_metrics.as_ref().unwrap();

    match table {
        &WindowsMetrics::Version3(ref table) => {
            assert_eq!(table.panose, &[2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert_eq!(stringify(&table.achVendID), "ADBE");
            assert_eq!(table.usBreakChar, 32);
        },
        _ => unreachable!(),
    }
}

fn open(name: &str) -> File {
    let path = PathBuf::from("tests/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    File::open(&path).unwrap()
}

fn stringify<T>(data: &[T]) -> &str {
    use std::{mem, slice, str};
    unsafe {
        let length = data.len() * mem::size_of::<T>();
        let bytes = slice::from_raw_parts(data as *const _ as *const _, length);
        str::from_utf8_unchecked(bytes)
    }
}
