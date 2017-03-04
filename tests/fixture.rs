use std::path::PathBuf;

pub enum Fixture {
    CFF,
    TTF,
    VariableCFF,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::CFF => "tests/fixtures/SourceSerifPro-Regular.otf",
            Fixture::TTF => "tests/fixtures/OpenSans-Italic.ttf",
            Fixture::VariableCFF => "tests/fixtures/AdobeVFPrototype.otf",
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
            Fixture::VariableCFF => match table {
                _ => unreachable!(),
            },
        }
    }
}
