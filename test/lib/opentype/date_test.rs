#![feature(macro_rules)]

extern crate opentype;

macro_rules! date(
    ($year:expr, $month:expr, $day:expr) => (
        opentype::Date { year: $year, month: $month, day: $day }
    )
)

#[test]
fn new_test() {
    assert_eq!(opentype::Date::new(       -42), date!(1904, 01, 01));
    assert_eq!(opentype::Date::new(         0), date!(1904, 01, 01));
    assert_eq!(opentype::Date::new(   2678399), date!(1904, 01, 31));
    assert_eq!(opentype::Date::new(   2678400), date!(1904, 02, 01));
    assert_eq!(opentype::Date::new(   5184000), date!(1904, 03, 01));
    assert_eq!(opentype::Date::new(3491078399), date!(2014, 08, 16));
    assert_eq!(opentype::Date::new(3491078400), date!(2014, 08, 17));
}
