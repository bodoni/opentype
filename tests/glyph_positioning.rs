#[macro_use]
mod support;

mod adobe_vf_prototype {
    use opentype::tables::glyph_positioning::GlyphPositioning;
    use opentype::value::Read;

    #[test]
    fn features() {
        let GlyphPositioning { features, .. } =
            ok!(Read::read(&mut setup!(AdobeVFPrototypeTTF, "GPOS")));
        let tags = features
            .headers
            .iter()
            .map(|header| header.tag)
            .collect::<Vec<_>>();
        assert_eq!(tags, tags![b"kern", b"size"]);
    }
}

mod crimson_text {
    use opentype::tables::glyph_positioning::GlyphPositioning;
    use opentype::value::Read;

    #[test]
    fn features() {
        let GlyphPositioning { features, .. } = ok!(Read::read(&mut setup!(CrimsonText, "GPOS")));
        let tags = features
            .headers
            .iter()
            .map(|header| header.tag)
            .collect::<Vec<_>>();
        assert_eq!(tags, tags![b"kern", b"mark", b"mkmk"]);
    }
}

mod source_serif {
    use opentype::layout::Language;
    use opentype::layout::Script;
    use opentype::tables::glyph_positioning::{GlyphPositioning, PairAdjustment, Type};
    use opentype::value::Read;

    #[test]
    fn features() {
        let GlyphPositioning { features, .. } =
            ok!(Read::read(&mut setup!(SourceSerifPro, "GPOS")));
        let tags = features
            .headers
            .iter()
            .map(|header| header.tag)
            .collect::<Vec<_>>();
        assert_eq!(
            tags,
            tags![
                b"kern", b"kern", b"kern", b"kern", b"kern", b"size", b"size", b"size", b"size",
                b"size",
            ],
        );
        let lookups = features
            .records
            .iter()
            .map(|record| record.lookup_index_count)
            .collect::<Vec<_>>();
        assert_eq!(lookups, &[1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn lookups() {
        let GlyphPositioning { lookups, .. } = ok!(Read::read(&mut setup!(SourceSerifPro, "GPOS")));
        assert_eq!(lookups.records.len(), 1);
        let record = &lookups.records[0];
        assert!(record.mark_filtering_set.is_none());
        assert_eq!(record.tables.len(), 2);
        match &record.tables[0] {
            &Type::PairAdjustment(PairAdjustment::Format1(ref table)) => {
                assert_eq!(table.record_count, 65);
            }
            _ => unreachable!(),
        }
        match &record.tables[1] {
            &Type::PairAdjustment(PairAdjustment::Format2(ref table)) => {
                assert_eq!(table.class1_count, 99);
                assert_eq!(table.class2_count, 95);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn scripts() {
        let GlyphPositioning { scripts, .. } = ok!(Read::read(&mut setup!(SourceSerifPro, "GPOS")));
        let tags = scripts
            .headers
            .iter()
            .map(|header| header.tag)
            .collect::<Vec<_>>();
        assert_eq!(tags, tags![b"DFLT", b"latn"]);
        assert!(scripts.get(Script::Default).is_some());
        assert!(scripts.get(Script::Latin).is_some());
        let tags = scripts
            .records
            .iter()
            .map(|record| {
                record
                    .language_headers
                    .iter()
                    .map(|header| header.tag)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(tags, &[vec![], tags![b"AZE ", b"CRT ", b"TRK "]]);
        let record = &scripts.records[0];
        assert!(record.default_language.is_some());
        assert_eq!(record.language_count, 0);
        let record = &scripts.records[1];
        assert_eq!(record.language_count, 3);
        assert!(record.get(Language::Turkish).is_some());
    }
}
