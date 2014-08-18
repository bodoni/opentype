#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate date;
extern crate opentype;

use support::*;

#[test]
fn parse_cff_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let font = opentype::parse(&mut file).unwrap();

    assert_eq!(font.format, opentype::CFF);
}

#[test]
fn parse_font_header_test() {
    macro_rules! date(
        ($year:expr, $month:expr, $day:expr) => (
            date::Date { year: $year, month: $month, day: $day }
        )
    )

    let mut file = open_fixture!("SourceSerifPro-Bold.otf");
    let font = opentype::parse(&mut file).unwrap();

    assert_eq!(font.units_per_em, 1000);

    assert_eq!(font.created_on, date!(2014, 4, 27));
    assert_eq!(font.updated_on, date!(2014, 4, 27));

    assert_eq!(font.style.bold, true);
    assert_eq!(font.style.italic, false);
    assert_eq!(font.style.condensed, false);
    assert_eq!(font.style.extended, false);
}

#[test]
fn parse_maximal_profile_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let font = opentype::parse(&mut file).unwrap();

    assert_eq!(font.glyph_count, 545);
}
