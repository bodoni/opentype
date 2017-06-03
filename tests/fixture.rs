use std::path::PathBuf;

pub enum Fixture {
    CFF,
    TTF,
    VariableCFF,
    VariableTTF,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::CFF => "tests/fixtures/SourceSerifPro-Regular.otf".into(),
            Fixture::TTF => "tests/fixtures/OpenSans-Italic.ttf".into(),
            Fixture::VariableCFF => "tests/fixtures/AdobeVFPrototype.otf".into(),
            Fixture::VariableTTF => "tests/fixtures/Gingham.ttf".into(),
        }
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::CFF => {
                match table {
                    "GPOS" => 60412,
                    "GSUB" => 57648,
                    _ => unreachable!(),
                }
            },
            Fixture::TTF => {
                match table {
                    "GDEF" => 206348,
                    _ => unreachable!(),
                }
            },
            Fixture::VariableCFF => {
                match table {
                    _ => unreachable!(),
                }
            },
            Fixture::VariableTTF => {
                match table {
                    _ => unreachable!(),
                }
            },
        }
    }
}
