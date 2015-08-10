use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaximumProfile {
    Version05(MaximumProfile05),
    Version10(MaximumProfile10),
}

table!(MaximumProfile05 {
    version   (Fixed ),
    numGlyphs (USHORT),
});

table!(MaximumProfile10 {
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
