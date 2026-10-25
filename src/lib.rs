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

The catalog does not retain executable formula pointers:

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

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "engine")]
pub mod calculator;

#[cfg(feature = "engine")]
pub mod const_eval;

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

#[allow(dead_code)]
#[cfg(feature = "engine")]
mod test {
    use crate::{
        AbilityId, ChampionId, Ctx, ItemId, RuneId,
        const_eval::{ConstDamage, ConstDamages, ConstEnemy, ConstInput, ConstOutput},
        model::{AbilityLevels, Attacks, BasicStats, Dragons, Modifiers, PlayerStats, ResistShred},
    };

    const CHAMPION_ID: ChampionId = ChampionId::Neeko;
    const OUT: ConstOutput<{ CHAMPION_ID.number_of_abilities() }, 6, 1> = ConstInput {
        champion_id: CHAMPION_ID,
        items: [
            ItemId::NashorsTooth,
            ItemId::RabadonsDeathcap,
            ItemId::HextechRocketbelt,
            ItemId::ZhonyasHourglass,
            ItemId::SorcerersShoes,
            ItemId::VoidStaff,
        ],
        runes: [RuneId::Electrocute],
        rune_exceptions: [(RuneId::GatheringStorm, 4)],
        item_exceptions: [(ItemId::Dragonheart, 3)],
        ability_levels: AbilityLevels::default(),
        stats: None,
        dragons: Dragons::default(),
        stacks: 0,
        level: core::num::NonZeroU8::new(18).unwrap(),
        is_mega_gnar: false,
        enemy: ConstEnemy {
            champion_id: ChampionId::Aatrox,
            items: [ItemId::ForceOfNature],
            item_exceptions: [(ItemId::Dragonheart, 4)],
            stats: None,
            stacks: 0,
            level: core::num::NonZeroU8::new(18).unwrap(),
            is_mega_gnar: false,
        },
    }
    .eval();

    const ATTACKS: Attacks = OUT.damages.attacks;
    const ABILITIES: &[(AbilityId, i32)] = &OUT.damages.abilities;
    const ITEMS_DMG: &[ConstDamage<ItemId>] = &OUT.damages.items;
    const RUNES_DMG: &[ConstDamage<RuneId>] = &OUT.damages.runes;
    const CTX: Ctx = OUT.ctx;
    const STATS: PlayerStats = OUT.stats;
    const BASE_STATS: BasicStats = OUT.base_stats;
    const BONUS_STATS: BasicStats = OUT.bonus_stats;
    const SHRED: ResistShred = OUT.shred;
    const MODIFIERS: Modifiers = OUT.modifiers;
    const SIML: [(
        ItemId,
        ConstDamages<{ ABILITIES.len() }, { ITEMS_DMG.len() }, { RUNES_DMG.len() }>,
    ); ItemId::L_SIML] = OUT.siml;
}
