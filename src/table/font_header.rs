use primitive::*;

spec! {
    pub FontHeader {
        version            (Fixed       ),
        fontRevision       (Fixed       ),
        checkSumAdjustment (ULONG       ),
        magicNumber        (ULONG       ),
        flags              (USHORT      ),
        unitsPerEm         (USHORT      ),
        created            (LONGDATETIME),
        modified           (LONGDATETIME),
        xMin               (SHORT       ),
        yMin               (SHORT       ),
        xMax               (SHORT       ),
        yMax               (SHORT       ),
        macStyle           (USHORT      ),
        lowestRecPPEM      (USHORT      ),
        fontDirectionHint  (SHORT       ),
        indexToLocFormat   (SHORT       ),
        glyphDataFormat    (SHORT       ),
    }
}
