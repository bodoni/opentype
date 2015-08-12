use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaximumProfile {
    Version05(MaximumProfile05),
    Version10(MaximumProfile10),
}

spec! {
    #[derive(Copy)]
    pub MaximumProfile05 {
        version   (Fixed ),
        numGlyphs (UShort),
    }
}

spec! {
    #[derive(Copy)]
    pub MaximumProfile10 {
        version               (Fixed ),
        numGlyphs             (UShort),
        maxPoints             (UShort),
        maxContours           (UShort),
        maxCompositePoints    (UShort),
        maxCompositeContours  (UShort),
        maxZones              (UShort),
        maxTwilightPoints     (UShort),
        maxStorage            (UShort),
        maxFunctionDefs       (UShort),
        maxInstructionDefs    (UShort),
        maxStackElements      (UShort),
        maxSizeOfInstructions (UShort),
        maxComponentElements  (UShort),
        maxComponentDepth     (UShort),
    }
}

impl MaximumProfile {
    pub fn glyphs(&self) -> usize {
        match self {
            &MaximumProfile::Version05(ref profile) => profile.numGlyphs as usize,
            &MaximumProfile::Version10(ref profile) => profile.numGlyphs as usize,
        }
    }
}
