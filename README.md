# OpenType [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for OpenType fonts. It might be helpful to have a
look at a higher-level parser called [`font`][font], which internally relies on
this package.

## [Documentation][doc]

## Example

```rust
extern crate opentype;
extern crate truetype;

let path = "SourceSerifPro-Regular.otf";
let mut file = std::fs::File::open(path).unwrap();
let opentype::File { fonts, .. } = opentype::File::read(&mut file).unwrap();

assert_eq!(fonts[0].font_header(&mut file).unwrap().unwrap().units_per_em, 1000);
assert_eq!(fonts[0].horizontal_header(&mut file).unwrap().unwrap().ascender, 918);
let strings = match fonts[0].naming_table(&mut file).unwrap().unwrap() {
    truetype::NamingTable::Format0(ref table) => table.strings().unwrap(),
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
