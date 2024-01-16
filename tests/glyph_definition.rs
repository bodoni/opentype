#[macro_use]
mod support;

use opentype::layout::Class;
use opentype::tables::glyph_definition::{GlyphDefinition, Header};
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
            assert_eq!(table.count, 1);
            assert_eq!(table.records[0].start_glyph_id, 0);
            assert_eq!(table.records[0].end_glyph_id, 937);
        }
        _ => unreachable!(),
    }
    assert!(table.attachments.is_none());
    match &table.ligatures {
        &Some(ref table) => assert_eq!(table.count, 0),
        _ => unreachable!(),
    }
}
