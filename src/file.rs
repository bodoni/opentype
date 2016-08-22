use std::io::{Read, Seek};
use std::ops::Deref;
use truetype::{Result, Tag, Tape, q32};

use Font;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Read a file.
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<File> {
        if Tag::from(try!(Tape::peek::<q32>(tape))) == Tag(*b"ttcf") {
            raise!("TrueType collections are not supported yet");
        }
        Ok(File { fonts: vec![try!(Font::read(tape))] })
    }
}

impl Deref for File {
    type Target = [Font];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}

#[cfg(test)]
mod tests {
    use File;

    const CFF: &'static str = "tests/fixtures/SourceSerifPro-Regular.otf";
    const TTF: &'static str = "tests/fixtures/OpenSans-Italic.ttf";

    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn cff() {
        use postscript::compact::FontSet;

        let mut reader = ok!(::std::fs::File::open(CFF));
        let file = ok!(File::read(&mut reader));
        let _ = ok!(ok!(file[0].take::<_, FontSet>(&mut reader)));
    }

    #[test]
    fn ttf() {
        use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

        let mut reader = ok!(::std::fs::File::open(TTF));
        let file = ok!(File::read(&mut reader));
        let font_header = ok!(ok!(file[0].take::<_, FontHeader>(&mut reader)));
        let maximum_profile = ok!(ok!(file[0].take::<_, MaximumProfile>(&mut reader)));
        let glyph_mapping = ok!(ok!(file[0].take_given::<_, GlyphMapping>(&mut reader,
                                                                          (&font_header,
                                                                           &maximum_profile))));
        let _ = ok!(ok!(file[0].take_given::<_, GlyphData>(&mut reader, &glyph_mapping)));
    }
}
