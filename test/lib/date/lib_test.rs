#![feature(macro_rules)]

extern crate date;
use date::Date;

macro_rules! date(
    ($year:expr, $month:expr, $day:expr) => (
        Date { year: $year, month: $month, day: $day }
    )
)

#[test]
fn since_1904_test() {
    assert_eq!(Date::since(1904,          0), date!(1904, 01, 01));
    assert_eq!(Date::since(1904,    2678399), date!(1904, 01, 31));
    assert_eq!(Date::since(1904,    2678400), date!(1904, 02, 01));
    assert_eq!(Date::since(1904,    5184000), date!(1904, 03, 01));
    assert_eq!(Date::since(1904, 3491078399), date!(2014, 08, 16));
    assert_eq!(Date::since(1904, 3491078400), date!(2014, 08, 17));
}
