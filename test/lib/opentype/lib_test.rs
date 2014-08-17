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
fn parse_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let font = opentype::parse(&mut file).unwrap();

    assert_eq!(font.format, opentype::CFF);
    assert_eq!(font.units_per_em, 1000);

    assert_eq!(font.created_on, date!(2014, 4, 27));
    assert_eq!(font.updated_on, date!(2014, 4, 27));
}
