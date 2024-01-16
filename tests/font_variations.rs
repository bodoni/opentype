#[macro_use]
mod support;

use opentype::tables::font_variations::FontVariations;
use opentype::Value;
use truetype::Tag;

#[test]
fn table() {
    let table: FontVariations = ok!(Value::read(&mut setup!(AdobeVFPrototypeTTF, "fvar")));
    assert_eq!(table.header.major_version, 1);
    assert_eq!(table.header.minor_version, 0);
    assert_eq!(table.header.axis_count, 2);
    assert_eq!(table.axis_records[0].tag, Tag(*b"wght"));
    assert_eq!(f32::from(table.axis_records[0].min_value), 200.0);
    assert_eq!(f32::from(table.axis_records[0].max_value), 900.0);
}
