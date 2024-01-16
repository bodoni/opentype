#[macro_use]
mod support;

use opentype::tables::color_palettes::{ColorPalettes, Header};
use opentype::Value;

#[test]
fn table() {
    let table: ColorPalettes = ok!(Value::read(&mut setup!(NotoColorEmoji, "CPAL")));
    let header = match table.header {
        Header::Version0(value) => value,
    };
    assert_eq!(header.version, 0);
}
