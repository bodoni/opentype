use primitive::*;

#[derive(Debug)]
pub enum MaxProfile {
    Version05(MaxProfile05),
    Version10(MaxProfile10),
}

table!(MaxProfile05 {
    version   (Fixed ),
    numGlyphs (USHORT),
});

table!(MaxProfile10 {
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
});
