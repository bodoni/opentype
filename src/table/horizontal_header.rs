use primitive::*;

spec! {
    #[derive(Copy)]
    pub HorizontalHeader {
        version             (Fixed ),
        Ascender            (FWord ),
        Descender           (FWord ),
        LineGap             (FWord ),
        advanceWidthMax     (UFWord),
        minLeftSideBearing  (FWord ),
        minRightSideBearing (FWord ),
        xMaxExtent          (FWord ),
        caretSlopeRise      (Short ),
        caretSlopeRun       (Short ),
        caretOffset         (Short ),
        reserved1           (Short ),
        reserved2           (Short ),
        reserved3           (Short ),
        reserved4           (Short ),
        metricDataFormat    (Short ),
        numberOfHMetrics    (UShort),
    }
}
