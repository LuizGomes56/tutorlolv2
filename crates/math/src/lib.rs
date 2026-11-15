#![no_std]
#![cfg_attr(
    not(feature = "engine"),
    doc = r#"
Catalog and transport types remain available without the `engine` feature.
The executable calculation API requires `engine`:

```compile_fail
use math::calculator;
```

```compile_fail
let _ = math::ChampionId::eval;
```

```compile_fail
let _ = math::ItemId::eval;
```

```compile_fail
let _ = math::RuneId::eval;
```

The catalog does not retain executable formula pointers:

```compile_fail
let _ = |champion: math::Champion| champion.closures;
```

```compile_fail
let _ = |item: math::Item| item.melee;
```

```compile_fail
let _ = |rune: math::Rune| rune.ranged;
```
"#
)]

extern crate alloc;

/// Highlighted documentation of every formula, in any [`codec::Markup`].
#[cfg(feature = "formula-docs")]
pub mod formula_docs;

#[cfg(any(feature = "yew", feature = "formula-docs"))]
pub mod yew;

#[cfg(feature = "engine")]
pub mod calculator;

#[cfg(feature = "engine")]
pub mod helpers;

#[cfg(feature = "engine")]
pub mod realtime;

pub mod bitset;
pub mod libgen;
pub mod model;
pub mod riot;

pub use libgen::*;
pub use types::*;
