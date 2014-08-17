#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate opentype;

use support::*;

macro_rules! date(
    ($year:expr, $month:expr, $day:expr) => (
        opentype::Date { year: $year, month: $month, day: $day }
    )
)

#[test]
fn date_new_test() {
    assert_eq!(opentype::Date::new(       -42), date!(1904, 01, 01));
    assert_eq!(opentype::Date::new(         0), date!(1904, 01, 01));
    assert_eq!(opentype::Date::new(   2678399), date!(1904, 01, 31));
    assert_eq!(opentype::Date::new(   2678400), date!(1904, 02, 01));
    assert_eq!(opentype::Date::new(   5184000), date!(1904, 03, 01));
    assert_eq!(opentype::Date::new(3491078399), date!(2014, 08, 16));
    assert_eq!(opentype::Date::new(3491078400), date!(2014, 08, 17));
}

#[test]
fn parse_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let font = opentype::parse(&mut file).unwrap();

    assert_eq!(font.format, opentype::CFF);
    assert_eq!(font.units_per_em, 1000);
    assert_eq!(font.created_at, date!(2014, 4, 27));
    assert_eq!(font.updated_at, date!(2014, 4, 27));
}
