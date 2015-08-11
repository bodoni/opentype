use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaximumProfile {
    Version05(MaximumProfile05),
    Version10(MaximumProfile10),
}

spec! {
    pub MaximumProfile05 {
        version   (Fixed ),
        numGlyphs (USHORT),
    }
}

spec! {
    pub MaximumProfile10 {
        version               (Fixed ),
        numGlyphs             (USHORT),
        maxPoints             (USHORT),
        maxContours           (USHORT),
        maxCompositePoints    (USHORT),
        maxCompositeContours  (USHORT),
        maxZones              (USHORT),
        maxTwilightPoints     (USHORT),
        maxStorage            (USHORT),
        maxFunctionDefs       (USHORT),
        maxInstructionDefs    (USHORT),
        maxStackElements      (USHORT),
        maxSizeOfInstructions (USHORT),
        maxComponentElements  (USHORT),
        maxComponentDepth     (USHORT),
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
