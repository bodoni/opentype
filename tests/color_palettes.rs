#[macro_use]
mod support;

mod noto_color_emoji {
    use opentype::tables::color_palettes::{ColorPalettes, Header};
    use opentype::value::Read;

    #[test]
    fn read() {
        let table: ColorPalettes = ok!(Read::read(&mut setup!(NotoColorEmoji, "CPAL")));
        let header = match table.header {
            Header::Version0(ref value) => value,
            _ => unreachable!(),
        };
        assert_eq!(header.version, 0);
        let values = table
            .iter()
            .map(|palette| {
                palette
                    .map(|color| {
                        format!(
                            "#{:02x}{:02x}{:02x}{:02x}",
                            color.red, color.green, color.blue, color.alpha,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].len(), 5895);
        assert_eq!(
            values[0].iter().take(10).collect::<Vec<_>>(),
            &[
                "#000000ff",
                "#00000dff",
                "#000066ff",
                "#000088ff",
                "#00008bff",
                "#000095ff",
                "#0000ffff",
                "#000101ff",
                "#000200ff",
                "#000202ff",
            ],
        );
    }
}

mod kalnia_glaze {
    use opentype::tables::color_palettes::ColorPalettes;
    use opentype::value::Read;

    #[test]
    fn read() {
        let _: ColorPalettes = ok!(Read::read(&mut setup!(KalniaGlaze, "CPAL")));
    }
}
