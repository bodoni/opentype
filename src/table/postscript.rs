use primitive::*;

table!(PostScript {
    version            (Fixed),
    italicAngle        (Fixed),
    underlinePosition  (FWORD),
    underlineThickness (FWORD),
    isFixedPitch       (ULONG),
    minMemType42       (ULONG),
    maxMemType42       (ULONG),
    minMemType1        (ULONG),
    maxMemType1        (ULONG),
});
