use crate::{
    calculator::InferStats,
    helpers::{ability_id_mod, get_enemy_full_state, get_eval_ctx},
    model::{
        AbilityLevels, Attacks, BasicStats, ConstDamageKind, DamageModifiers, Dragons, EnemyState,
        EnemyStats, Modifiers, RangeDamage, ResistShred, RiotFormulas, SelfState, SimpleStats,
        Stats, ValueException,
    },
};
use tutorlolv2_gen::{
    AbilityId, AttackType, ChampionId, Closure, Ctx, ITEM_CACHE, ItemId, ItemsBitSet, RUNE_CACHE,
    RuneId, RunesBitSet, TypeMetadata,
};

pub const fn get_items_data_const<const N: usize, const L: usize>(
    items: &ItemsBitSet,
    attack_type: AttackType,
) -> ConstDamageKind<ItemId, N, L> {
    assert!(L == N << 1);
    unsafe {
        let mut metadata: [TypeMetadata<ItemId>; N] = core::mem::zeroed();
        let mut closures: [Closure; L] = core::mem::zeroed();

        let mut i = 0;
        let mut j = 0;

        let mut iter = items.iter_const();

        while let Some(index) = iter.next_const() {
            let item = ITEM_CACHE[index as usize];
            let slice = match attack_type {
                AttackType::Ranged => item.ranged,
                AttackType::Melee => item.melee,
            };

            metadata[i] = item.metadata;
            closures[j] = slice[0];
            closures[j + 1] = slice[1];

            i += 1;
            j += 2;
        }

        assert!(i == N);
        assert!(j == L);

        ConstDamageKind { metadata, closures }
    }
}

pub const fn get_runes_data_const<const N: usize, const L: usize>(
    runes: &RunesBitSet,
    attack_type: AttackType,
) -> ConstDamageKind<RuneId, N, L> {
    assert!(L == N << 1);
    unsafe {
        let mut metadata: [TypeMetadata<RuneId>; N] = core::mem::zeroed();
        let mut closures: [Closure; L] = core::mem::zeroed();

        let mut i = 0;
        let mut j = 0;

        let mut iter = runes.iter_const();

        while let Some(index) = iter.next_const() {
            let rune = RUNE_CACHE[index as usize];
            let slice = match attack_type {
                AttackType::Ranged => rune.ranged,
                AttackType::Melee => rune.melee,
            };

            metadata[i] = rune.metadata;
            closures[j] = slice[0];
            closures[j + 1] = slice[1];

            i += 1;
            j += 2;
        }

        assert!(i == N);
        assert!(j == L);

        ConstDamageKind { metadata, closures }
    }
}

pub const fn const_ability_id_eval_damage<const N: usize>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    champion_id: ChampionId,
    modifiers: Modifiers,
) -> [(AbilityId, i32); N] {
    let mut result: [_; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;

    while i < N {
        let TypeMetadata {
            kind,
            damage_type,
            attributes,
        } = champion_id.metadata()[i];

        let modifier = ability_id_mod(kind, damage_type, modifiers);
        let damage = (modifier * champion_id.eval(ctx, kind)) as i32;

        onhit.inc_attr(attributes, damage);
        result[i] = (kind, damage);

        i += 1;
    }
    result
}

pub const fn eval_item_damage_const<const N: usize>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    item_ids: [ItemId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [ConstDamage<ItemId>; N] {
    let mut result: [_; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;

    while i < N {
        let item_id = item_ids[i];
        let TypeMetadata {
            damage_type,
            attributes,
            kind,
        } = item_id.metadata();
        let modifier = modifiers.damages.modifier(damage_type);
        let damages = item_id.eval(ctx, attack_type);

        let min = (modifier * damages[0]) as i32;
        let max = (modifier * damages[1]) as i32;

        result[i] = ConstDamage { kind, min, max };

        onhit.inc_attr(attributes, min);
        onhit.inc_attr(attributes, max);

        i += 1;
    }

    result
}

pub const fn eval_rune_damage_const<const N: usize>(
    ctx: &Ctx,
    rune_ids: [RuneId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [ConstDamage<RuneId>; N] {
    let mut result: [_; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;

    while i < N {
        let rune_id = rune_ids[i];
        let TypeMetadata {
            kind, damage_type, ..
        } = rune_id.metadata();
        let modifier = modifiers.damages.modifier(damage_type);
        let damages = rune_id.eval(ctx, attack_type);

        let min = (modifier * damages[0]) as i32;
        let max = (modifier * damages[1]) as i32;

        result[i] = ConstDamage { kind, min, max };

        i += 1;
    }

    result
}

#[derive(Clone, Copy)]
pub struct ConstInput<
    const I: usize,
    const R: usize,
    const EI: usize,
    const EIX: usize,
    const RE: usize,
    const IE: usize,
> {
    pub champion_id: ChampionId,
    pub items: [ItemId; I],
    pub runes: [RuneId; R],
    pub rune_exceptions: [(RuneId, u32); RE],
    pub item_exceptions: [(ItemId, u32); IE],
    pub ability_levels: AbilityLevels,
    pub stats: Option<Stats<f32>>,
    pub dragons: Dragons,
    pub stacks: u32,
    pub level: u8,
    pub is_mega_gnar: bool,
    pub enemy: ConstEnemy<EI, EIX>,
}

#[derive(Clone, Copy)]
pub struct ConstEnemy<const N: usize, const EIE: usize> {
    pub champion_id: ChampionId,
    pub items: [ItemId; N],
    pub item_exceptions: [(ItemId, u32); EIE],
    pub stats: Option<EnemyStats<f32>>,
    pub stacks: u32,
    pub level: u8,
    pub is_mega_gnar: bool,
}

#[derive(Clone, Copy)]
pub struct ConstDamage<T> {
    pub kind: T,
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Copy)]
pub struct ConstOutput<const A: usize, const I: usize, const R: usize> {
    pub attacks: Attacks,
    pub abilities: [(AbilityId, i32); A],
    pub items: [ConstDamage<ItemId>; I],
    pub runes: [ConstDamage<RuneId>; R],
    pub ctx: Ctx,
    pub stats: Stats<f32>,
    pub base_stats: BasicStats<f32>,
    pub bonus_stats: BasicStats<f32>,
    pub shred: ResistShred,
    pub modifiers: Modifiers,
}

impl<
    const I: usize,
    const R: usize,
    const EI: usize,
    const EIX: usize,
    const RE: usize,
    const IE: usize,
> ConstInput<I, R, EI, EIX, RE, IE>
{
    pub const fn eval<const A: usize>(self) -> ConstOutput<A, I, R> {
        let Self {
            champion_id,
            items,
            runes,
            rune_exceptions,
            item_exceptions,
            ability_levels,
            stats,
            dragons,
            stacks,
            level,
            is_mega_gnar,
            enemy,
        } = self;

        if !matches!(champion_id, ChampionId::Gnar) && is_mega_gnar {
            panic!("`is_mega_gnar` is only valid if champion is Gnar")
        }

        let base_stats = BasicStats::base_stats(champion_id, level, is_mega_gnar);

        let mut modifiers = Modifiers::default();

        let stats = match stats {
            Some(s) => s,
            None => Stats::infer(InferStats {
                item_exceptions: &ValueException::pack_items(&item_exceptions),
                rune_exceptions: &ValueException::pack_runes(&rune_exceptions),
                items: &items,
                runes: &runes,
                modifiers: &mut modifiers,
                dragons,
                ability_levels,
                stacks,
                level,
                champion_id,
                is_mega_gnar,
            }),
        };

        let bonus_stats = stats.bonus_stats(base_stats);

        let adaptive_type =
            match RiotFormulas::adaptive_type(bonus_stats.attack_damage, stats.ability_power) {
                Some(v) => v,
                None => champion_id.adaptive_type(),
            };

        let self_state = SelfState {
            stacks: stacks as _,
            ability_levels,
            current_stats: stats,
            bonus_stats,
            base_stats,
            level,
            adaptive_type,
        };

        let shred = ResistShred::new(&stats);

        let enemy_state = {
            let ConstEnemy {
                champion_id,
                item_exceptions,
                stats,
                stacks,
                level,
                is_mega_gnar,
                ..
            } = enemy;

            if !matches!(enemy.champion_id, ChampionId::Gnar) && is_mega_gnar {
                panic!("`is_mega_gnar` is only valid if enemy champion is Gnar")
            }

            EnemyState {
                current_stats: stats,
                base_stats: SimpleStats::infer(enemy.champion_id, enemy.level, is_mega_gnar),
                items: &enemy.items,
                stacks,
                champion_id,
                earth_dragons: dragons.enemy_earth_dragons,
                level,
                item_exceptions: &ValueException::pack_items(&item_exceptions),
            }
        };

        let attack_type = if is_mega_gnar {
            AttackType::Ranged
        } else {
            champion_id.attack_type()
        };

        let enemy = get_enemy_full_state(enemy_state, shred, false);
        let ctx = get_eval_ctx(&self_state, &enemy);

        let modifiers = Modifiers {
            damages: DamageModifiers {
                adaptive_type: self_state.adaptive_type,
                physical_mod: modifiers.damages.physical_mod
                    * enemy.armor_values.modifier
                    * enemy.modifiers.physical_mod,
                magic_mod: modifiers.damages.magic_mod
                    * enemy.magic_values.modifier
                    * enemy.modifiers.magic_mod,
                true_mod: modifiers.damages.true_mod * enemy.modifiers.true_mod,
                global_mod: modifiers.damages.global_mod * enemy.modifiers.global_mod,
            },
            ..modifiers
        };

        let mut onhit = RangeDamage::default();

        let abilities = const_ability_id_eval_damage::<A>(&ctx, &mut onhit, champion_id, modifiers);
        let items = eval_item_damage_const(&ctx, &mut onhit, items, attack_type, modifiers);
        let runes = eval_rune_damage_const(&ctx, runes, attack_type, modifiers);

        let attacks = Attacks::new(&ctx, onhit, modifiers.damages.physical_mod);

        ConstOutput {
            ctx,
            attacks,
            abilities,
            items,
            runes,
            stats,
            base_stats,
            bonus_stats,
            shred,
            modifiers,
        }
    }
}

/// Does a very similar work as the function `calculator` in the main module, but
/// in constant context to debug purposes.
///
/// # Example
///
/// ```rs
/// const CHAMPION_ID: ChampionId = ChampionId::Neeko;
/// const ITEMS: [ItemId; 1] = [ItemId::NashorsTooth];
/// const RUNES: [RuneId; 1] = [RuneId::Electrocute];
///
/// static OUT: ConstOutput<
///     { CHAMPION_ID.number_of_abilities() },
///     { ITEMS.len() } /* 1 */,
///     { RUNES.len() } /* 1 */,
/// > =
///     ConstInput {
///         champion_id: CHAMPION_ID,
///         items: ITEMS /* [ItemId::NashorsTooth] */,
///         runes: RUNES /* [RuneId::Electrocute] */,
///         rune_exceptions: [(RuneId::GatheringStorm, 4)],
///         item_exceptions: [(ItemId::Dragonheart, 3)],
///         ability_levels: AbilityLevels::default(),
///         stats: None,
///         dragons: Dragons::default(),
///         stacks: 0,
///         level: 18,
///         is_mega_gnar: false,
///         enemy: ConstEnemy {
///             champion_id: ChampionId::Aatrox,
///             items: [ItemId::ForceOfNature],
///             item_exceptions: [(ItemId::Dragonheart, 4)],
///             stats: None,
///             stacks: 0,
///             level: 18,
///             is_mega_gnar: false,
///         },
///     }
///     .eval();
/// ```
pub const fn calculator<
    const A: usize,
    const I: usize,
    const R: usize,
    const EI: usize,
    const EIX: usize,
    const RE: usize,
    const IE: usize,
>(
    input: ConstInput<I, R, EI, EIX, RE, IE>,
) -> ConstOutput<A, I, R> {
    input.eval()
}
