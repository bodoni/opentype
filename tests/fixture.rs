use std::path::PathBuf;

pub enum Fixture {
    AdobeVFPrototype,
    Gingham,
    OpenSans,
    SourceSerifPro,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::AdobeVFPrototype => "tests/fixtures/AdobeVFPrototype.otf".into(),
            Fixture::Gingham => "tests/fixtures/Gingham.ttf".into(),
            Fixture::OpenSans => "tests/fixtures/OpenSans-Italic.ttf".into(),
            Fixture::SourceSerifPro => "tests/fixtures/SourceSerifPro-Regular.otf".into(),
        }
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::AdobeVFPrototype => {
                match table {
                    _ => unreachable!(),
                }
            },
            Fixture::Gingham => {
                match table {
                    _ => unreachable!(),
                }
            },
            Fixture::OpenSans => {
                match table {
                    "GDEF" => 206348,
                    _ => unreachable!(),
                }
            },
            Fixture::SourceSerifPro => {
                match table {
                    "GPOS" => 60412,
                    "GSUB" => 57648,
                    _ => unreachable!(),
                }
            },
        }
    }
}
