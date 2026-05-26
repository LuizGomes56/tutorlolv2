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
    use crate::{
        AbilityId, AdaptiveType, ChampionId, ComboElement, Ctx, DamageType, ItemId, RuneId,
        bitset::{ItemsBitSet, RunesBitSet, bitset},
        calculator::{InferStats, infer_champion_stats},
        const_eval,
        helpers::{get_damaging_items, get_damaging_runes, get_enemy_full_state, get_eval_ctx},
        model::{
            AbilityLevels, Attacks, BasicStats, Damages, Dragons, EnemyFullState, EnemyState,
            EnemyStats, Modifiers, RangeDamage, ResistShred, RiotFormulas, SelfState, SimpleStats,
            Stats, ValueException,
        },
    };

    #[test]
    pub fn const_eval() {
        const N: usize = CHAMPION_ID.number_of_abilities();
        const CHAMPION_ID: ChampionId = ChampionId::Yasuo;
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

        const ADAPTIVE_TYPE: AdaptiveType =
            RiotFormulas::adaptive_type(BONUS_STATS.attack_damage, STATS.ability_power).unwrap();

        const SELF_STATE: SelfState = SelfState {
            stacks: STACKS,
            ability_levels: ABILITY_LEVELS,
            current_stats: STATS,
            bonus_stats: BONUS_STATS,
            base_stats: BASE_STATS,
            level: LEVEL,
            adaptive_type: ADAPTIVE_TYPE,
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
        const MODIFIERS: Modifiers = Modifiers::new(&CTX, ADAPTIVE_TYPE);

        const ABILITIES: [AbilityId; N] = CHAMPION_ID.ability_ids();

        const ABILITY_DAMAGES: [i32; N] =
            const_eval::const_ability_id_eval_damage(&CTX, CHAMPION_ID, MODIFIERS);

        const ITEMS: [ItemId; 2] = [ItemId::NashorsTooth, ItemId::CloakOfAgility];

        const ITEM_DAMAGES: [i32; ITEMS.len() << 1] =
            const_eval::eval_item_damage_const(&CTX, ITEMS, CHAMPION_ID.attack_type(), MODIFIERS);

        const RUNES: [RuneId; 1] = [RuneId::Electrocute];
        const RUNE_DAMAGES: [i32; RUNES.len() << 1] =
            const_eval::eval_rune_damage_const(&CTX, RUNES, CHAMPION_ID.attack_type(), MODIFIERS);

        const ATTACKS: Attacks =
            Attacks::new(&CTX, RangeDamage::default(), MODIFIERS.damages.physical_mod);

        const I0_DMG: [f32; 2] = ItemId::NashorsTooth.eval(&CTX, CHAMPION_ID.attack_type());
        const R0_DMG: [f32; 2] = RuneId::Electrocute.eval(&CTX, CHAMPION_ID.attack_type());
    }

    #[derive(Clone, Copy)]
    pub struct SnapshotInput<
        const ITEMS: usize,
        const RUNES: usize,
        const E_ITEMS: usize,
        const E_ITEM_EXC: usize,
        const RE: usize,
        const IE: usize,
    > {
        pub champion_id: ChampionId,
        pub items: [ItemId; ITEMS],
        pub runes: [RuneId; RUNES],
        pub level: u8,
        pub stacks: u32,
        pub enemy: EnemyData<E_ITEMS, E_ITEM_EXC>,
        rune_exceptions: [(RuneId, u32); RE],
        item_exceptions: [(ItemId, u32); IE],
    }

    #[derive(Clone, Copy)]
    pub struct DamageSnapshot<
        const ABILITIES: usize,
        const ITEM_DMGS: usize,
        const RUNE_DMGS: usize,
    > {
        pub attacks: Attacks,
        pub ability_damages: [i32; ABILITIES],
        pub item_damages: [i32; ITEM_DMGS],
        pub rune_damages: [i32; RUNE_DMGS],
    }

    pub const fn build_snapshot<
        const A: usize,
        const I: usize,
        const ID: usize,
        const R: usize,
        const RD: usize,
        const EI: usize,
        const EIE: usize,
        const RE: usize,
        const IE: usize,
    >(
        input: SnapshotInput<I, R, EI, EIE, RE, IE>,
    ) -> DamageSnapshot<A, ID, RD> {
        assert!(ID == I << 1);
        assert!(RD == R << 1);

        let base_stats = BasicStats::base_stats(
            input.champion_id,
            input.level,
            matches!(input.champion_id, ChampionId::Gnar),
        );

        let ability_levels = AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        };

        let dragons = Dragons::default();

        let mut modifiers = Modifiers::default();

        let stats = infer_champion_stats(InferStats {
            item_exceptions: &{
                let mut result: [ValueException; IE] = unsafe { core::mem::zeroed() };

                let mut i = 0;
                while i < input.item_exceptions.len() {
                    let (item_id, v) = input.item_exceptions[i];
                    result[i] = ValueException::pack_item_id(item_id, v);
                    i += 1;
                }

                result
            },
            rune_exceptions: &{
                let mut result: [ValueException; RE] = unsafe { core::mem::zeroed() };

                let mut i = 0;
                while i < input.rune_exceptions.len() {
                    let (rune_id, v) = input.rune_exceptions[i];
                    result[i] = ValueException::pack_rune_id(rune_id, v);
                    i += 1;
                }

                result
            },
            items: &input.items,
            runes: &input.runes,
            modifiers: &mut modifiers,
            dragons,
            ability_levels,
            stacks: input.stacks as _,
            level: input.level,
            champion_id: input.champion_id,
            is_mega_gnar: matches!(input.champion_id, ChampionId::Gnar),
        });

        let bonus_stats = stats.bonus_stats(base_stats);

        let adaptive_type =
            match RiotFormulas::adaptive_type(bonus_stats.attack_damage, stats.ability_power) {
                Some(atype) => atype,
                None => input.champion_id.adaptive_type(),
            };

        let self_state = SelfState {
            stacks: input.stacks as _,
            ability_levels,
            current_stats: stats,
            bonus_stats,
            base_stats,
            level: input.level,
            adaptive_type,
        };

        let shred = ResistShred::new(&stats);

        let enemy_state = EnemyState {
            current_stats: input.enemy.current_stats,
            base_stats: SimpleStats::infer(
                input.enemy.champion_id,
                input.enemy.level,
                matches!(input.enemy.champion_id, ChampionId::Gnar),
            ),
            items: &input.enemy.items,
            stacks: input.enemy.stacks,
            champion_id: input.enemy.champion_id,
            earth_dragons: input.enemy.earth_dragons,
            level: input.enemy.level,
            item_exceptions: &{
                let mut result: [ValueException; EIE] = unsafe { core::mem::zeroed() };

                let mut i = 0;
                while i < input.enemy.item_exceptions.len() {
                    let (item_id, v) = input.enemy.item_exceptions[i];
                    result[i] = ValueException::pack_item_id(item_id, v);
                    i += 1;
                }

                result
            },
        };

        let enemy = get_enemy_full_state(enemy_state, shred, false);

        let ctx = get_eval_ctx(&self_state, &enemy);

        let modifiers = Modifiers::new(&ctx, adaptive_type);

        DamageSnapshot {
            attacks: Attacks::new(&ctx, RangeDamage::default(), modifiers.damages.physical_mod),

            ability_damages: const_eval::const_ability_id_eval_damage(
                &ctx,
                input.champion_id,
                modifiers,
            ),

            item_damages: const_eval::eval_item_damage_const(
                &ctx,
                input.items,
                input.champion_id.attack_type(),
                modifiers,
            ),

            rune_damages: const_eval::eval_rune_damage_const(
                &ctx,
                input.runes,
                input.champion_id.attack_type(),
                modifiers,
            ),
        }
    }

    #[derive(Clone, Copy)]
    pub struct EnemyData<const N: usize, const EIE: usize> {
        pub current_stats: Option<EnemyStats<f32>>,
        pub items: [ItemId; N],
        pub stacks: u32,
        pub champion_id: ChampionId,
        pub earth_dragons: u16,
        pub item_exceptions: [(ItemId, u32); EIE],
        pub level: u8,
    }

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
    const SNAPSHOT: DamageSnapshot<
        { CHAMPION_ID.number_of_abilities() },
        { ITEMS.len() << 1 },
        { RUNES.len() << 1 },
    > = build_snapshot(SnapshotInput {
        champion_id: CHAMPION_ID,
        items: ITEMS,
        runes: RUNES,
        rune_exceptions: [(RuneId::GatheringStorm, 4)],
        item_exceptions: [(ItemId::Dragonheart, 3)],
        level: 18,
        stacks: 0,
        enemy: EnemyData {
            champion_id: ChampionId::Aatrox,
            level: 18,
            items: [ItemId::ForceOfNature],
            stacks: 0,
            earth_dragons: 0,
            item_exceptions: [(ItemId::Dragonheart, 4)],
            current_stats: None,
        },
    });
}
