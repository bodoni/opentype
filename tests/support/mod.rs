#![allow(dead_code, unused_macros)]

use std::fs::File;
use std::path::PathBuf;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (crate::support::setup(crate::support::Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (crate::support::setup(crate::support::Fixture::$fixture, Some($table)));
);

macro_rules! tags(
    ($($name:expr,)*) => (vec![$(::truetype::Tag(*$name),)*]);
    ($($name:expr),*) => (tags!($($name,)*));
);

pub enum Fixture {
    AdobeVFPrototypeCFF,
    AdobeVFPrototypeTTF,
    KaushanScript,
    OpenSans,
    SourceSerifPro,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::AdobeVFPrototypeCFF => "tests/fixtures/AdobeVFPrototype.otf",
            Fixture::AdobeVFPrototypeTTF => "tests/fixtures/AdobeVFPrototype.ttf",
            Fixture::KaushanScript => "tests/fixtures/KaushanScript-Regular.ttf",
            Fixture::OpenSans => "tests/fixtures/OpenSans-Italic.ttf",
            Fixture::SourceSerifPro => "tests/fixtures/SourceSerifPro-Regular.otf",
        }
        .into()
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::AdobeVFPrototypeCFF => match table {
                _ => unreachable!(),
            },
            Fixture::AdobeVFPrototypeTTF => match table {
                _ => unreachable!(),
            },
            Fixture::OpenSans => match table {
                "GDEF" => 206348,
                _ => unreachable!(),
            },
            Fixture::KaushanScript => match table {
                _ => unreachable!(),
            },
            Fixture::SourceSerifPro => match table {
                "GPOS" => 60412,
                "GSUB" => 57648,
                _ => unreachable!(),
            },
        }
    }
}

pub fn setup(fixture: Fixture, table: Option<&str>) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(
        table.map(|table| fixture.offset(table)).unwrap_or(0)
    )));
    file
}
