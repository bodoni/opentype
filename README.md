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
use truetype::NamingTable;

let path = "SourceSerifPro-Regular.otf";
let file = File::open(path).unwrap();
let font = &file[0];

assert_eq!(font.font_header.as_ref().unwrap().units_per_em, 1000);
assert_eq!(font.horizontal_header.as_ref().unwrap().ascender, 918);
let strings = match font.naming_table {
    Some(NamingTable::Format0(ref table)) => table.strings().unwrap(),
    _ => unreachable!(),
};
assert_eq!(&strings[1], "Source Serif Pro");
assert_eq!(&strings[9], "Frank Grießhammer");
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
