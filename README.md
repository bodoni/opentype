# OpenType [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for OpenType fonts.

## [Documentation][doc]

## Example

```rust
extern crate opentype;
extern crate truetype;

use opentype::File;
use truetype::compound::NamingTable;

let path = "SourceSerifPro-Regular.otf";
let font = Font::open(path).unwrap();

match font.font_header {
    Some(ref table) => assert_eq!(table.unitsPerEm, 1000),
    _ => unreachable!(),
}
match font.horizontal_header {
    Some(ref table) => assert_eq!(table.Ascender, 918),
    _ => unreachable!(),
}
match font.naming_table {
    Some(NamingTable::Format0(ref table)) => {
        let strings = table.strings().unwrap();
        assert_eq!(&strings[1], "Source Serif Pro");
        assert_eq!(&strings[9], "Frank Grießhammer");
    },
    _ => unreachable!(),
}
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
