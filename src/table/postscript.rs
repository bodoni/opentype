use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostScript {
    Version10(PostScript10),
    Version30(PostScript30),
}

spec! {
    #[derive(Copy)]
    pub PostScript10 {
        version            (Fixed),
        italicAngle        (Fixed),
        underlinePosition  (FWORD),
        underlineThickness (FWORD),
        isFixedPitch       (ULONG),
        minMemType42       (ULONG),
        maxMemType42       (ULONG),
        minMemType1        (ULONG),
        maxMemType1        (ULONG),
    }
}

pub type PostScript30 = PostScript10;
