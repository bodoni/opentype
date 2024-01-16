# OpenType [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a parser for OpenType fonts. It might be helpful to have a
look at a higher-level parser called [`font`][font], which internally relies on
this package.

## Example

```rust
use opentype::truetype::tables::names::{Names, NameID};
use opentype::truetype::tables::{FontHeader, HorizontalHeader};
use opentype::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "SourceSerifPro-Regular.otf";
let mut tape = ok!(std::fs::File::open(path));
let File { mut fonts } = ok!(File::read(&mut tape));

let font_header: FontHeader = ok!(ok!(fonts[0].take(&mut tape)));
assert_eq!(font_header.units_per_em, 1000);

let horizontal_header: HorizontalHeader = ok!(ok!(fonts[0].take(&mut tape)));
assert_eq!(horizontal_header.ascender, 918);

let names: Names = ok!(ok!(fonts[0].take(&mut tape)));
let names = names
    .iter()
    .map(|((name_id, _), value)| (name_id, value))
    .collect::<std::collections::HashMap<_, _>>();
assert_eq!(ok!(names[&NameID::FullFontName].as_ref()), "Source Serif Pro");
assert_eq!(ok!(names[&NameID::DesignerName].as_ref()), "Frank Grießhammer");
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[font]: https://github.com/bodoni/font

[build-img]: https://github.com/bodoni/opentype/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/bodoni/opentype/actions/workflows/build.yml
[documentation-img]: https://docs.rs/opentype/badge.svg
[documentation-url]: https://docs.rs/opentype
[package-img]: https://img.shields.io/crates/v/opentype.svg
[package-url]: https://crates.io/crates/opentype
