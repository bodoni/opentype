extern crate opentype;
extern crate postscript;
extern crate truetype;

use opentype::File;

#[macro_use]
mod common;

#[test]
fn cff() {
    use postscript::compact1::FontSet;

    let mut reader = setup!(SourceSerifPro);
    let file = ok!(File::read(&mut reader));
    let _ = ok!(ok!(file[0].take::<_, FontSet>(&mut reader)));
}

#[test]
fn cff_variable() {
    use opentype::GlyphSubstitution;

    let mut reader = setup!(AdobeVFPrototype);
    let file = ok!(File::read(&mut reader));
    let _ = ok!(ok!(file[0].take::<_, GlyphSubstitution>(&mut reader)));
}

#[test]
fn ttf() {
    use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

    let mut reader = setup!(OpenSans);
    let file = ok!(File::read(&mut reader));
    let font_header = ok!(ok!(file[0].take::<_, FontHeader>(&mut reader)));
    let maximum_profile = ok!(ok!(file[0].take::<_, MaximumProfile>(&mut reader)));
    let glyph_mapping = ok!(ok!(
        file[0].take_given::<_, GlyphMapping>(&mut reader, (&font_header, &maximum_profile))
    ));
    let _ = ok!(ok!(
        file[0].take_given::<_, GlyphData>(&mut reader, &glyph_mapping)
    ));
}

#[test]
#[cfg_attr(not(feature = "ignore-invalid-checksums"), should_panic)]
fn ttf_corrupted() {
    use truetype::FontHeader;

    let mut reader = setup!(KaushanScript);
    let file = ok!(File::read(&mut reader));
    let _ = ok!(ok!(file[0].take::<_, FontHeader>(&mut reader)));
}

#[test]
fn ttf_variable() {
    use opentype::GlyphSubstitution;

    let mut reader = setup!(Gingham);
    let file = ok!(File::read(&mut reader));
    let _ = ok!(ok!(file[0].take::<_, GlyphSubstitution>(&mut reader)));
}
