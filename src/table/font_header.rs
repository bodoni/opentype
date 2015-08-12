use primitive::*;

spec! {
    #[derive(Copy)]
    pub FontHeader {
        version            (Fixed       ),
        fontRevision       (Fixed       ),
        checkSumAdjustment (ULong       ),
        magicNumber        (ULong       ),
        flags              (UShort      ),
        unitsPerEm         (UShort      ),
        created            (LongDateTime),
        modified           (LongDateTime),
        xMin               (Short       ),
        yMin               (Short       ),
        xMax               (Short       ),
        yMax               (Short       ),
        macStyle           (UShort      ),
        lowestRecPPEM      (UShort      ),
        fontDirectionHint  (Short       ),
        indexToLocFormat   (Short       ),
        glyphDataFormat    (Short       ),
    }
}
