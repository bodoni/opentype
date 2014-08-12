#![feature(globs, phase, macro_rules)]

#[phase(link, plugin)]
extern crate support;

extern crate opentype;

use support::*;

#[test]
fn parse_test() {
    let mut file = open_fixture!("SourceSerifPro-Regular.otf");
    let fonts = opentype::parse(&mut file).unwrap();

    assert_eq!(fonts.len(), 1);
}
