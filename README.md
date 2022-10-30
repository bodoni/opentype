# OpenType [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a parser for OpenType fonts. It might be helpful to have a
look at a higher-level parser called [`font`][font], which internally relies on
this package.

## Example

```rust
extern crate opentype;

use opentype::Font;
use opentype::truetype::{FontHeader, HorizontalHeader, NamingTable};

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "SourceSerifPro-Regular.otf";
let mut reader = ok!(std::fs::File::open(path));
let font = ok!(Font::read(&mut reader));

let font_header: FontHeader = ok!(ok!(font.take(&mut reader)));
assert!(font_header.units_per_em == 1000);

let horizontal_header: HorizontalHeader = ok!(ok!(font.take(&mut reader)));
assert!(horizontal_header.ascender == 918);

let naming_table: NamingTable = ok!(ok!(font.take(&mut reader)));
match naming_table {
    NamingTable::Format0(ref table) => {
        let strings = ok!(table.strings());
        assert!(&strings[1] == "Source Serif Pro");
        assert!(&strings[9] == "Frank Grießhammer");
    },
    _ => unreachable!(),
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[font]: https://github.com/bodoni/font

[build-img]: https://github.com/bodoni/opentype/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/opentype/actions/workflows/build.yml
[documentation-img]: https://docs.rs/opentype/badge.svg
[documentation-url]: https://docs.rs/opentype
[package-img]: https://img.shields.io/crates/v/opentype.svg
[package-url]: https://crates.io/crates/opentype
