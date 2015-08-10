use primitive::*;

table!(HorizontalHeader {
    version             (Fixed   ),
    Ascender            (FWORD   ),
    Descender           (FWORD   ),
    LineGap             (FWORD   ),
    advanceWidthMax     (UFWORD  ),
    minLeftSideBearing  (FWORD   ),
    minRightSideBearing (FWORD   ),
    xMaxExtent          (FWORD   ),
    caretSlopeRise      (SHORT   ),
    caretSlopeRun       (SHORT   ),
    caretOffset         (SHORT   ),
    reserved1           (SHORT   ),
    reserved2           (SHORT   ),
    reserved3           (SHORT   ),
    reserved4           (SHORT   ),
    metricDataFormat    (SHORT   ),
    numberOfHMetrics    (USHORT  ),
});
