extern crate opentype;

use opentype::Font;
use std::fs::{self, File};
use std::path::PathBuf;

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
