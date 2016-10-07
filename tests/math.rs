use opentype::math_table::Math;
use opentype::math_table::constants::{ Percentage, DesignUnits };
use truetype::Value;

#[test]
fn table() {
    let math: Math = ok!(Value::read(&mut setup!(MATH, "MATH")));
    assert_eq!(math.header.constants_offset, 10);
    assert_eq!(math.header.glyph_info_offset, 224);
    assert_eq!(math.header.variants_offset, 17034);

    assert_eq!(math.constants.axis_height.value, DesignUnits(250));
    assert_eq!(math.constants.script_percent_scale_down, Percentage(70));
}