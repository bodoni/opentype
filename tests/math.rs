use opentype::mathematics::Mathematics;
use truetype::Value;

#[test]
fn table() {
    let math: Mathematics = ok!(Value::read(&mut setup!(MATH, "MATH")));
    assert_eq!(math.header.constants_offset, 10);
    assert_eq!(math.header.glyph_info_offset, 224);
    assert_eq!(math.header.variants_offset, 17034);
}