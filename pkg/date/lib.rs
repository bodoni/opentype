#![feature(macro_rules)]

extern crate time;

/// A representation of a day in the Gregorian calendar.
#[deriving(Default, PartialEq, Eq, Ord, Show)]
pub struct Date {
    /// The year.
    pub year: u32,
    /// The month.
    pub month: u8,
    /// The day.
    pub day: u8,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        macro_rules! cmp(
            ($one:expr, $two:expr) => (
                if $one > $two {
                    return Some(std::cmp::Greater);
                } else if $one < $two {
                    return Some(std::cmp::Less);
                }
            );
        )

        cmp!(self.year, other.year);
        cmp!(self.month, other.month);
        cmp!(self.day, other.day);

        Some(std::cmp::Equal)
    }
}

impl Date {
    /// Create a date by the specified year, month, and day.
    #[inline]
    pub fn new(year: u32, month: u8, day: u8) -> Date {
        Date { year: year, month: month, day: day }
    }

    /// Return the UTC date specified in seconds counting from the Unix epoch.
    pub fn at_utc(seconds: i64) -> Date {
        let time = time::at_utc(time::Timespec { sec: seconds, nsec: 0 });
        Date::new(time.tm_year as u32 + 1900, time.tm_mon as u8 + 1, time.tm_mday as u8)
    }

    /// Return the UTC date specified in seconds counting from January 1, 1904.
    #[inline]
    pub fn at_utc_1904(seconds: i64) -> Date {
        Date::at_utc(seconds - 2082844800)
    }
}

#[cfg(test)]
mod test {
    macro_rules! date(
        ($year:expr, $month:expr, $day:expr) => (::Date::new($year, $month, $day));
    )

    #[test]
    fn eq() {
        assert_eq!(date!(2014, 8, 19), date!(2014, 8, 19));
    }

    #[test]
    fn ord() {
        assert!(date!(2014, 8, 19) < date!(2014, 8, 20));
        assert!(date!(2014, 8, 19) > date!(2014, 8, 18));
        assert!(date!(2014, 8, 19) < date!(2014, 9, 19));
        assert!(date!(2014, 8, 19) > date!(2014, 7, 19));
        assert!(date!(2014, 8, 19) < date!(2015, 8, 19));
        assert!(date!(2014, 8, 19) > date!(2013, 8, 19));
    }

    #[test]
    fn at_utc_since_1904() {
        macro_rules! at(($seconds:expr) => (::Date::at_utc_1904($seconds));)

        assert_eq!(at!(0), date!(1904, 1, 1));
        assert_eq!(at!(2678399), date!(1904, 1, 31));
        assert_eq!(at!(2678400), date!(1904, 2, 1));
        assert_eq!(at!(5184000), date!(1904, 3, 1));
        assert_eq!(at!(3491078399), date!(2014, 8, 16));
        assert_eq!(at!(3491078400), date!(2014, 8, 17));
    }
}
