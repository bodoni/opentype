use std::default;

#[deriving(Show)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day
    }
}

impl default::Default for Date {
    fn default() -> Date {
        Date { year: 1904, month: 1, day: 1 }
    }
}

impl Date {
    pub fn new(seconds_since_1904: i64) -> Date {
        if seconds_since_1904 <= 0 {
            return default::Default::default();
        }

        let mut seconds = seconds_since_1904 as u32;

        macro_rules! days_to_seconds(
            ($days:expr) => ($days * 24 * 60 * 60);
            ($($days:expr),+) => ([$($days * 24 * 60 * 60),+]);
        )

        let mut year: u32 = 1904;
        let mut leap;

        loop {
            leap =
                (year % 400) == 0 ||
                (year % 100) != 0 &&
                (year %   4) == 0;

            let year_seconds: u32 =
                days_to_seconds!(if leap { 366 } else { 365 });

            if year_seconds > seconds {
                break;
            }

            year += 1;
            seconds -= year_seconds;
        }

        let mut month: u8 = 1;

        let month_seconds: &[u32] = days_to_seconds!(
            31, if leap { 29 } else { 28 }, 31,
            30, 31, 30, 31, 30, 30, 31, 30, 31
        );

        for month_second in month_seconds.iter() {
            if *month_second > seconds {
                break;
            }

            month += 1;
            seconds -= *month_second;
        }

        let day = (seconds / days_to_seconds!(1)) as u8 + 1;

        Date { year: year, month: month, day: day }
    }
}
