use opentype::File;

#[test]
fn cff() {
    use postscript::compact::FontSet;

    let mut reader = setup!(CFF);
    let file = ok!(File::read(&mut reader));
    let _ = ok!(ok!(file[0].take::<_, FontSet>(&mut reader)));
}

#[test]
fn ttf() {
    use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

    let mut reader = setup!(TTF);
    let file = ok!(File::read(&mut reader));
    let font_header = ok!(ok!(file[0].take::<_, FontHeader>(&mut reader)));
    let maximum_profile = ok!(ok!(file[0].take::<_, MaximumProfile>(&mut reader)));
    let glyph_mapping = ok!(ok!(file[0].take_given::<_, GlyphMapping>(&mut reader, (&font_header,
                                                                       &maximum_profile))));
    let _ = ok!(ok!(file[0].take_given::<_, GlyphData>(&mut reader, &glyph_mapping)));
}
