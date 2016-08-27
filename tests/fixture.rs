use std::path::PathBuf;

pub enum Fixture {
    CFF,
    TTF,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::CFF => "tests/fixtures/SourceSerifPro-Regular.otf",
            Fixture::TTF => "tests/fixtures/OpenSans-Italic.ttf",
        }.into()
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::CFF => match table {
                "GPOS" => 60412,
                "GSUB" => 57648,
                _ => unreachable!(),
            },
            Fixture::TTF => match table {
                "GDEF" => 206348,
                _ => unreachable!(),
            },
        }
    }
}
