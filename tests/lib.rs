extern crate opentype;
extern crate postscript;
extern crate truetype;

use std::fs::File;
use std::io::{Seek, SeekFrom};

mod fixture;

use fixture::Fixture;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (::setup(::fixture::Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (::setup(::fixture::Fixture::$fixture, Some($table)));
);

macro_rules! tags(($($name:expr),*) => (vec![$(::truetype::Tag(*$name)),*]));

mod file;
mod glyph_definition;
mod glyph_positioning;
mod glyph_substitution;

fn setup(fixture: Fixture, table: Option<&str>) -> File {
    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(table.map(|table| fixture.offset(table)).unwrap_or(0))));
    file
}
