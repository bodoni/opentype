#[macro_use]
mod support;

use opentype::glyph_definition::{GlyphDefinition, Header};
use opentype::layout::Class;
use opentype::Value;

#[test]
fn table() {
    let table: GlyphDefinition = ok!(Value::read(&mut setup!(OpenSans, "GDEF")));
    match &table.header {
        &Header::Version1(..) => {}
        _ => unreachable!(),
    }
    match &table.glyph_class {
        &Some(Class::Format2(ref table)) => {
            assert!(table.range_count == 1);
            assert!(table.ranges[0].start == 0);
            assert!(table.ranges[0].end == 937);
        }
        _ => unreachable!(),
    }
    assert!(table.attachments.is_none());
    match &table.ligatures {
        &Some(ref table) => assert!(table.count == 0),
        _ => unreachable!(),
    }
}
