#![no_std]

pub use calculator::calculator;
pub use realtime::realtime;
pub use tutorlolv2_gen::{
    AbilityId, AbilityName, AdaptiveType, AttackType, Attrs, ChampionId, Ctx, DamageType, GameMap,
    ItemId, Position, RuneId,
};

extern crate alloc;

pub mod calculator;
pub mod const_eval;
pub mod helpers;
pub mod model;
pub mod realtime;
pub mod riot;

pub mod constants {
    pub use tutorlolv2_gen::{CHAMPION_CACHE, ITEM_CACHE, RUNE_CACHE};
}

pub mod champions {
    pub use tutorlolv2_gen::champions::*;
}

pub mod items {
    pub use tutorlolv2_gen::items::*;
}

pub mod runes {
    pub use tutorlolv2_gen::runes::*;
}

pub mod bitset {
    pub use tutorlolv2_gen::{
        bitset,
        bitset::{ItemsBitSet, RunesBitSet},
    };
}

#[allow(dead_code, unused_imports)]
mod test {
    use tutorlolv2_gen::DamageType;

    use crate::{
        calculator::{InferStats, infer_champion_stats},
        model::Dragons,
    };

    #[test]
    pub fn const_eval() {
        use crate::{
            AbilityId, ChampionId, Ctx, ItemId, RuneId,
            bitset::{ItemsBitSet, RunesBitSet, bitset},
            const_eval,
            helpers::{get_damaging_items, get_damaging_runes, get_enemy_full_state, get_eval_ctx},
            model::{
                AbilityLevels, BasicStats, EnemyFullState, EnemyState, EnemyStats, Modifiers,
                RangeDamage, ResistShred, SelfState, SimpleStats, Stats, ValueException,
            },
        };

        const N: usize = CHAMPION_ID.number_of_abilities();
        const CHAMPION_ID: ChampionId = ChampionId::Neeko;
        const BASE_STATS: BasicStats<f32> =
            BasicStats::base_stats(CHAMPION_ID, LEVEL, matches!(CHAMPION_ID, ChampionId::Gnar));
        // const STATS: Stats<f32> = Stats {
        //     ability_power: 500.0,
        //     armor: BASE_STATS.armor,
        //     armor_penetration_flat: 0.0,
        //     armor_penetration_percent: 0.0,
        //     attack_damage: BASE_STATS.attack_damage,
        //     attack_speed: CHAMPION_ID.stats().attack_speed.base,
        //     crit_chance: 0.0,
        //     crit_damage: CHAMPION_ID.stats().crit_base * CHAMPION_ID.stats().crit_modifier,
        //     current_health: BASE_STATS.max_health,
        //     magic_penetration_flat: 0.0,
        //     magic_penetration_percent: 0.0,
        //     magic_resist: BASE_STATS.magic_resist,
        //     max_health: BASE_STATS.max_health,
        //     max_mana: BASE_STATS.max_mana,
        //     current_mana: BASE_STATS.max_mana,
        // };
        const DRAGONS: Dragons = Dragons::default();
        const STATS: Stats<f32> = infer_champion_stats(InferStats {
            item_exceptions: &[],
            rune_exceptions: &[],
            items: &ITEMS,
            runes: &RUNES,
            modifiers: &mut Modifiers::default(),
            dragons: DRAGONS,
            ability_levels: ABILITY_LEVELS,
            stacks: STACKS as _,
            level: LEVEL,
            champion_id: CHAMPION_ID,
            is_mega_gnar: matches!(CHAMPION_ID, ChampionId::Gnar),
        });
        const ABILITY_LEVELS: AbilityLevels = AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        };
        const LEVEL: u8 = 18;
        const STACKS: f32 = 0.0;
        const BONUS_STATS: BasicStats<f32> = STATS.bonus_stats(BASE_STATS);
        const SELF_STATE: SelfState = SelfState {
            stacks: STACKS,
            ability_levels: ABILITY_LEVELS,
            current_stats: STATS,
            bonus_stats: BONUS_STATS,
            base_stats: BASE_STATS,
            level: LEVEL,
            adaptive_type: CHAMPION_ID.adaptive_type(),
        };
        const SHRED: ResistShred = ResistShred::new(&STATS);

        const E_STATS: Option<EnemyStats<f32>> = None;
        const E_CHAMPION_ID: ChampionId = ChampionId::Aatrox;
        const E_ITEMS: &[ItemId] = &[ItemId::ForceOfNature, ItemId::JakShoTheProtean];
        const E_STACKS: u32 = 0;
        const E_LEVEL: u8 = 18;
        const E_EARTH_DRAGONS: u16 = 0;
        const E_ITEM_EXCEPTIONS: &[ValueException] =
            &[ValueException::pack_item_id(ItemId::Dragonheart, 2)];
        const E_BASE_STATS: SimpleStats<f32> = SimpleStats::infer(
            E_CHAMPION_ID,
            E_LEVEL,
            matches!(E_CHAMPION_ID, ChampionId::Gnar),
        );
        const ENEMY_STATE: EnemyState = EnemyState {
            current_stats: E_STATS,
            base_stats: E_BASE_STATS,
            items: E_ITEMS,
            stacks: E_STACKS,
            champion_id: E_CHAMPION_ID,
            earth_dragons: E_EARTH_DRAGONS,
            level: E_LEVEL,
            item_exceptions: E_ITEM_EXCEPTIONS,
        };
        const E_STATE: EnemyFullState = get_enemy_full_state(ENEMY_STATE, SHRED, false);

        const CTX: Ctx = get_eval_ctx(&SELF_STATE, &E_STATE);
        const MODIFIERS: Modifiers = Modifiers::new(&CTX, CHAMPION_ID.adaptive_type());

        const _ABILITIES: [AbilityId; N] = CHAMPION_ID.ability_ids();

        const ABILITY_DAMAGES: [i32; N] =
            const_eval::const_ability_id_eval_damage(&CTX, CHAMPION_ID, MODIFIERS);

        const ITEMS: [ItemId; 1] = [ItemId::NashorsTooth];
        const ITEM_DAMAGES: [i32; ITEMS.len() << 1] =
            const_eval::eval_item_damage_const(&CTX, ITEMS, CHAMPION_ID.attack_type(), MODIFIERS);

        const RUNES: [RuneId; 1] = [RuneId::Electrocute];
        const RUNE_DAMAGES: [i32; RUNES.len() << 1] =
            const_eval::eval_rune_damage_const(&CTX, RUNES, CHAMPION_ID.attack_type(), MODIFIERS);

        const I0_DMG: [f32; 2] = ItemId::NashorsTooth.eval(&CTX, CHAMPION_ID.attack_type());
        const R0_DMG: [f32; 2] = RuneId::Electrocute.eval(&CTX, CHAMPION_ID.attack_type());
    }
}
