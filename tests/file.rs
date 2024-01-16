#[macro_use]
mod support;

use opentype::File;

#[test]
fn cff_regular() {
    use opentype::postscript::compact1::FontSet;

    let mut tape = setup!(SourceSerifPro);
    let file = ok!(File::read(&mut tape));
    let _ = ok!(ok!(file[0].take::<_, FontSet>(&mut tape)));
}

#[test]
fn cff_variable() {
    use opentype::GlyphSubstitution;

    let mut tape = setup!(AdobeVFPrototypeCFF);
    let file = ok!(File::read(&mut tape));
    let _ = ok!(ok!(file[0].take::<_, GlyphSubstitution>(&mut tape)));
}

#[test]
#[cfg_attr(not(feature = "ignore-invalid-checksums"), should_panic)]
fn ttf_corrupted() {
    use opentype::truetype::tables::FontHeader;

    let mut tape = setup!(KaushanScript);
    let file = ok!(File::read(&mut tape));
    let _ = ok!(ok!(file[0].take::<_, FontHeader>(&mut tape)));
}

#[test]
fn ttf_regular() {
    use opentype::truetype::tables::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

    let mut tape = setup!(OpenSans);
    let file = ok!(File::read(&mut tape));
    let font_header = ok!(ok!(file[0].take::<_, FontHeader>(&mut tape)));
    let maximum_profile = ok!(ok!(file[0].take::<_, MaximumProfile>(&mut tape)));
    let glyph_mapping = ok!(ok!(
        file[0].take_given::<_, GlyphMapping>(&mut tape, (&font_header, &maximum_profile))
    ));
    let _ = ok!(ok!(
        file[0].take_given::<_, GlyphData>(&mut tape, &glyph_mapping)
    ));
}

#[test]
fn ttf_variable() {
    use opentype::GlyphSubstitution;

    let mut tape = setup!(AdobeVFPrototypeTTF);
    let file = ok!(File::read(&mut tape));
    let _ = ok!(ok!(file[0].take::<_, GlyphSubstitution>(&mut tape)));
}
