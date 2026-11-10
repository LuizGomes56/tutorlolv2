#![feature(
    const_index,
    const_trait_impl,
    fn_traits,
    unboxed_closures,
    generic_const_exprs,
    adt_const_params
)]
#![allow(incomplete_features)]
#![no_std]
#![cfg_attr(
    not(feature = "engine"),
    doc = r#"
Catalog and transport types remain available without the `engine` feature.
The executable calculation API requires `engine`:

```compile_fail
use tutorlolv2::calculator;
```

```compile_fail
let _ = tutorlolv2::ChampionId::eval;
```

```compile_fail
let _ = tutorlolv2::ItemId::eval;
```

```compile_fail
let _ = tutorlolv2::RuneId::eval;
```

The catalog does not retain executable formula callables:

```compile_fail
let _ = |champion: tutorlolv2::Champion| champion.closures;
```

```compile_fail
let _ = |item: tutorlolv2::Item| item.melee;
```

```compile_fail
let _ = |rune: tutorlolv2::Rune| rune.ranged;
```
"#
)]

extern crate alloc;

#[cfg(feature = "engine")]
mod callables {
    include!(concat!(env!("OUT_DIR"), "/callables.rs"));
}

/// Highlighted documentation of every formula, in any [`tutorlolv2_codec::Markup`].
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
pub use tutorlolv2_types::*;

/// Populated compile-time fixture used to compare generated callable paths.
#[doc(hidden)]
#[cfg(feature = "engine")]
pub mod test {
    use crate::{
        calculator::{ArrayOutputGame, calculator_array, calculator_layout},
        model::*,
        *,
    };
    pub const GAME: InputGame<'static> = InputGame {
        active_player: InputActivePlayer {
            runes: &[RuneId::Electrocute],
            rune_exceptions: &[],
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            current_health: 0.0,
            data: InputMinData {
                champion_id: ChampionId::Neeko,
                items: &[
                    ItemId::NashorsTooth,
                    ItemId::RabadonsDeathcap,
                    ItemId::SorcerersShoes,
                ],
                item_exceptions: &[],
                stats: None,
                stacks: 0,
                level: core::num::NonZeroU8::new(18).unwrap(),
                is_mega_gnar: false,
            },
        },
        enemy_players: &[InputMinData {
            champion_id: ChampionId::Aatrox,
            items: &[ItemId::ForceOfNature],
            item_exceptions: &[],
            stats: None,
            stacks: 0,
            level: core::num::NonZeroU8::new(18).unwrap(),
            is_mega_gnar: false,
        }],
        dragons: Dragons::default(),
        game_map: GameMap::Unknown,
    };
    pub const LAYOUT: DamageLayout = calculator_layout(GAME);
    pub const OUT: ArrayOutputGame<LAYOUT, 1> = calculator_array(GAME);
    pub const CTX: Ctx = OUT.damages.ctx(L_MSTR);
    pub const MODIFIERS: Modifiers = Modifiers::new(&CTX, OUT.current_player.adaptive_type);
}
