# OpenType [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for OpenType fonts.

## [Documentation][doc]

## Example

```rust
extern crate opentype;

use opentype::File;
use opentype::truetype::NamingTable;

let path = "SourceSerifPro-Regular.otf";
let font = Font::open(path).unwrap();

assert_eq!(file.font_header.as_ref().unwrap().unitsPerEm, 1000);
assert_eq!(file.horizontal_header.as_ref().unwrap().Ascender, 918);
let strings = match file.naming_table {
    Some(NamingTable::Format0(ref table)) => table.strings().unwrap(),
    _ => unreachable!(),
};
assert_eq!(&strings[1], "Source Serif Pro");
assert_eq!(&strings[9], "Frank Grießhammer");
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/opentype.svg
[version-url]: https://crates.io/crates/opentype
[status-img]: https://travis-ci.org/bodoni/opentype.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/opentype
[doc]: https://bodoni.github.io/opentype
