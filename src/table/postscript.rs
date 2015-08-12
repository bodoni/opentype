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
        underlinePosition  (FWord),
        underlineThickness (FWord),
        isFixedPitch       (ULong),
        minMemType42       (ULong),
        maxMemType42       (ULong),
        minMemType1        (ULong),
        maxMemType1        (ULong),
    }
}

pub type PostScript30 = PostScript10;
