#![allow(dead_code, unused_macros)]

use std::fs::File;
use std::path::PathBuf;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (crate::common::setup(crate::common::Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (crate::common::setup(crate::common::Fixture::$fixture, Some($table)));
);

macro_rules! tags(
    ($($name:expr,)*) => (vec![$(::truetype::Tag(*$name),)*]);
    ($($name:expr),*) => (tags!($($name,)*));
);

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
            Fixture::AdobeVFPrototype => match table {
                _ => unreachable!(),
            },
            Fixture::Gingham => match table {
                _ => unreachable!(),
            },
            Fixture::OpenSans => match table {
                "GDEF" => 206348,
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
