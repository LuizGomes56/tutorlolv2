#![no_std]

pub use calculator::calculator;
pub use realtime::realtime;
pub use tutorlolv2_gen::{
    AbilityId, AbilityName, AdaptiveType, AttackType, Attrs, ChampionId, ComboElement, Ctx,
    DamageType, GameMap, ItemId, Position, RuneId,
};

extern crate alloc;

pub mod calculator;
pub mod const_eval;
pub mod helpers;
pub mod model;
pub mod realtime;
pub mod riot;
pub use tutorlolv2_gen::*;

#[allow(dead_code, unused_imports)]
mod test {
    use crate::{
        AbilityId, AdaptiveType, AttackType, ChampionId, ComboElement, Ctx, DamageType, ItemId,
        L_SIML, RuneId, SIMULATED_ITEMS_ENUM, bitset,
        bitset::{ItemsBitSet, RunesBitSet},
        calculator::{InferStats, infer_champion_stats},
        const_eval::{self, ConstEnemy, ConstInput, ConstOutput},
        helpers::{get_damaging_items, get_damaging_runes, get_enemy_full_state, get_eval_ctx},
        model::{
            AbilityLevels, Attacks, BasicStats, Damages, Dragons, EnemyFullState, EnemyState,
            EnemyStats, Modifiers, RangeDamage, ResistShred, RiotFormulas, SelfState, SimpleStats,
            Stats, ValueException,
        },
    };

    const CHAMPION_ID: ChampionId = ChampionId::Neeko;
    const ITEMS: [ItemId; 6] = [
        ItemId::NashorsTooth,
        ItemId::RabadonsDeathcap,
        ItemId::HextechRocketbelt,
        ItemId::ZhonyasHourglass,
        ItemId::SorcerersShoes,
        ItemId::BansheesVeil,
    ];
    const RUNES: [RuneId; 1] = [RuneId::Electrocute];

    static OUT: ConstOutput<
        { CHAMPION_ID.number_of_abilities() },
        { ITEMS.len() },
        { RUNES.len() },
    > = ConstInput {
        champion_id: CHAMPION_ID,
        items: ITEMS,
        runes: RUNES,
        rune_exceptions: [(RuneId::GatheringStorm, 4)],
        item_exceptions: [(ItemId::Dragonheart, 3)],
        ability_levels: AbilityLevels::default(),
        stats: None,
        dragons: Dragons::default(),
        stacks: 0,
        level: 18,
        is_mega_gnar: false,
        enemy: ConstEnemy {
            champion_id: ChampionId::Aatrox,
            items: [ItemId::ForceOfNature],
            item_exceptions: [(ItemId::Dragonheart, 4)],
            stats: None,
            stacks: 0,
            level: 18,
            is_mega_gnar: false,
        },
    }
    .eval();
}
