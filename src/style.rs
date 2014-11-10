use std::fmt;

#[deriving(Default)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub condensed: bool,
    pub extended: bool,
}

impl fmt::Show for Style {
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> Result<(), fmt::FormatError> {

        macro_rules! append(
            ($chunk:expr to $target:expr if $condition:expr) => (
                if $condition {
                    if !$target.is_empty() {
                        $target.push_str(", ");
                    }
                    $target.push_str($chunk);
                }
            )
        )

        let mut line = String::new();

        append!("Bold"      to line if self.bold);
        append!("Italic"    to line if self.italic);
        append!("Condensed" to line if self.condensed);
        append!("Extended"  to line if self.extended);

        write!(formatter, "[{}]", line)
    }
}

impl Style {
    pub fn parse(&mut self, flags: u16) {
        self.bold      = ((1 << 0) & flags) > 0;
        self.italic    = ((1 << 1) & flags) > 0;
        self.condensed = ((1 << 5) & flags) > 0;
        self.extended  = ((1 << 6) & flags) > 0;
    }
}
