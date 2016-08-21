use std::io::{Read, Seek};
use std::ops::Deref;
use truetype::{Result, Tag, Tape, q32};

use font::Font;

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
        let mut file = ok!(::std::fs::File::open(CFF));
        let File { fonts, .. } = ok!(File::read(&mut file));
        let _ = ok!(ok!(fonts[0].font_set(&mut file)));
    }

    #[test]
    fn ttf() {
        let mut file = ok!(::std::fs::File::open(TTF));
        let File { fonts, .. } = ok!(File::read(&mut file));
        let font_header = ok!(ok!(fonts[0].font_header(&mut file)));
        let maximum_profile = ok!(ok!(fonts[0].maximum_profile(&mut file)));
        let glyph_mapping = ok!(ok!(fonts[0].glyph_mapping(&mut file,
                                                           (&font_header, &maximum_profile))));
        let _ = ok!(ok!(fonts[0].glyph_data(&mut file, &glyph_mapping)));
    }
}
