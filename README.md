# OpenType [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for OpenType fonts. It might be helpful to have a
look at a higher-level parser called [`font`][font], which internally relies on
this package.

## [Documentation][doc]

## Example

```rust
extern crate opentype;
extern crate truetype;

use opentype::File;
use truetype::{FontHeader, HorizontalHeader, NamingTable};

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "SourceSerifPro-Regular.otf";
let mut reader = ok!(std::fs::File::open(path));
let file = ok!(File::read(&mut reader));

let font_header: FontHeader = ok!(ok!(file[0].take(&mut reader)));
assert_eq!(font_header.units_per_em, 1000);

let horizontal_header: HorizontalHeader = ok!(ok!(file[0].take(&mut reader)));
assert_eq!(horizontal_header.ascender, 918);

let naming_table: NamingTable = ok!(ok!(file[0].take(&mut reader)));
if let NamingTable::Format0(ref table) = naming_table {
    let strings = ok!(table.strings());
    assert_eq!(&strings[1], "Source Serif Pro");
    assert_eq!(&strings[9], "Frank Grießhammer");
} else {
    unreachable!();
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[font]: https://github.com/bodoni/font

[doc]: https://bodoni.github.io/opentype
[status-img]: https://travis-ci.org/bodoni/opentype.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/opentype
[version-img]: https://img.shields.io/crates/v/opentype.svg
[version-url]: https://crates.io/crates/opentype
