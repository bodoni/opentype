# OpenType [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for OpenType fonts.

## [Documentation][doc]

## Example

```rust
use opentype::Font;
use opentype::table::NamingTable;
use std::fs::File;

let path = "SourceSerifPro-Regular.otf";
# let path = "tests/fixtures/SourceSerifPro-Regular.otf";
let mut file = File::open(path).unwrap();
let font = Font::read(&mut file).unwrap();

assert_eq!(font.font_header.unitsPerEm, 1000);
assert_eq!(font.horizontal_header.Ascender, 918);

let strings = match font.naming_table {
    NamingTable::Format0(ref table) => table.strings().unwrap(),
    _ => unreachable!(),
};

assert_eq!(&strings[1], "Source Serif Pro");
assert_eq!(&strings[9], "Frank Grießhammer");
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: http://stainless-steel.github.io/images/crates.svg
[version-url]: https://crates.io/crates/opentype
[status-img]: https://travis-ci.org/stainless-steel/opentype.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/opentype
[doc]: https://stainless-steel.github.io/opentype
