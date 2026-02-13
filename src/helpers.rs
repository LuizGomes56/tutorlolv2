//! This module exports several helper functions that help get
//! base stats, bonus stats, damaging abilities, damage against
//! jungle monsters, turrets, other champions, etc.
//!
//! These are functions that are used internally in the functions
//! [`crate::calculator`] and [`crate::realtime`], but if you want
//! to customize the damage calculation process, this is the place
//! to start
//!
//! Note that most functions have the `const` qualifier, which you
//! should use to debug and check if the calculated values are correct
//! without having to compile your project.
//!
//! Function pointers can't be called in const context, so if you still
//! want to do it, you can use the [`crate::const_eval`] module to
//! do it

use crate::calculator::MONSTER_RESISTS;
use crate::model::*;
use alloc::boxed::Box;
use core::mem::MaybeUninit;
use tutorlolv2_gen::*;

/// Rune [`RuneId::AxiomArcanist`] gives +12% bonus damage to `R`
/// if it deals single target damage. The -3% penalty is not yet
/// supported for area-damaging ultimates
pub const AXIOM_ARCANIST_BONUS_DAMAGE: f32 = 1.12;
pub const COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE: f32 = 1.08;
/// By 06/07/2025 Earth dragons give +5% resists
pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;
/// Despite the usual maximum level being 18, in
/// URF you can reach up to this constant's value
pub const URF_MAX_LEVEL: usize = 30;
/// Item [`ItemId::PlatedSteelcaps`] gives 12% damage reduction for basic attacks
pub const STEEL_CAPS_PROTECTION: f32 = 0.88;
/// Item [`ItemId::RanduinsOmen`] gives 30% damage reduction against critical hits
pub const RANDUIN_CRIT_PROTECTION: f32 = 0.7;
/// Items with Rocksolid passive give 20% damage reduction in some cases
pub const ROCKSOLID_PROTECTION: f32 = 0.8;
pub const SHOJIN_BONUS_DAMAGE: f32 = 1.12;
pub const SHADOWFLAME_BONUS_DAMAGE: f32 = 1.2;
pub const RIFTMAKER_BONUS_DAMAGE: f32 = 1.08;

/// Formula to get the bonus damage of the rune [`RuneId::LastStand`], where
/// missing health is a ratio of the current health and the maximum health.
/// ```rs
/// let missing_health = 1.0 - (
///     current_player_stats.current_health /
///         current_player_stats.health.max(1.0)
/// );
/// ```
/// Note that it uses [`f32::max`] to avoid division by zero.
///
/// - Current Health = 800
/// - Maximum Health = 1000
///
/// Then you're missing 200 health, which represents 20% of the total HP,
/// which should translate to 0.2.
/// Check the formula
/// `1.0 - (800.0 / 1000.0) = 0.2`
///
/// Also, this formula has a range from 1.0 to 1.11, since in game the
/// maximum damage increase is of `11%`
pub const fn get_last_stand(missing_health: f32) -> f32 {
    1.0 + (0.05 + 0.2 * (missing_health - 0.4)).clamp(0.0, 0.11)
}

/// Receives the number of Mountain dragons and returns a multiplier that will
/// be applied to increase some target's armor and magic resistences
pub const fn get_earth_multiplier(x: u16) -> f32 {
    1.0 + x as f32 * EARTH_DRAGON_MULTIPLIER
}

/// Receives the number of fire dragons and returns a number that can be multiplied
/// by the current ability power and attack damage to obtain the expected current
/// player's numeric value for those fields
pub const fn get_fire_multiplier(x: u16) -> f32 {
    1.0 + x as f32 * FIRE_DRAGON_MULTIPLIER
}

/// Constant array ordered based on the [`ChampionId`] when casted to a [`usize`]
/// index. The second inner array represents the base stats of that champion at
/// a given level that goes from 0 to [`URF_MAX_LEVEL`], where 0 represents the
/// level 1.
/// ```rs
/// let my_champion = ChampionId::Aatrox;
/// let my_level = 6;
/// BASE_STATS[my_champion_id as usize][6];
/// ```
pub static BASE_STATS: [[BasicStats<f32>; URF_MAX_LEVEL]; ChampionId::VARIANTS] = {
    let mut base_stats = [[BasicStats::<f32>::default(); URF_MAX_LEVEL]; ChampionId::VARIANTS];
    let mut champion_index = 0;
    while champion_index < ChampionId::VARIANTS {
        let champion_id = ChampionId::from_usize(champion_index).unwrap();
        let mut level = 0;
        while level < URF_MAX_LEVEL {
            let stats = &champion_id.cache().stats;
            macro_rules! mount_basic_stats {
                ($($field:ident),*) => {
                    BasicStats {
                        $(
                            $field: RiotFormulas::stat(
                                &stats.$field,
                                level as u8 + 1,
                            ),
                        )*
                    }
                };
            }
            base_stats[champion_index][level] = mount_basic_stats! {
                health,
                armor,
                magic_resist,
                attack_damage,
                mana
            };
            level += 1;
        }
        champion_index += 1;
    }
    base_stats
};

/// Constant sorted array containing a struct [`BasicStats<f32>`] of Mega Gnar.
/// Each index represents the base stats at each level. The maximum level is
/// defined by the constant [`URF_MAX_LEVEL`], which is over the usual maximum
/// level of 18 (or 20 by season 2026)
pub static MEGA_GNAR_BASE_STATS: [BasicStats<f32>; URF_MAX_LEVEL] = {
    let mut base_stats = BASE_STATS[ChampionId::Gnar as usize];
    let mut level = 0;
    while level < URF_MAX_LEVEL {
        type S = CachedChampionStatsMap;

        const MEGA_GNAR_HEALTH: S = S {
            flat: 100.0,
            per_level: 43.0,
        };
        const MEGA_GNAR_ARMOR: S = S {
            flat: 3.5,
            per_level: 3.0,
        };
        const MEGA_GNAR_MAGIC_RESIST: S = S {
            flat: 3.5,
            per_level: 3.5,
        };
        const MEGA_GNAR_ATTACK_DAMAGE: S = S {
            flat: 6.0,
            per_level: 2.5,
        };

        macro_rules! get_stat {
            ($field:ident) => {
                RiotFormulas::stat(&$field, level as u8 + 1)
            };
        }

        base_stats[level].health += get_stat!(MEGA_GNAR_HEALTH);
        base_stats[level].armor += get_stat!(MEGA_GNAR_ARMOR);
        base_stats[level].magic_resist += get_stat!(MEGA_GNAR_MAGIC_RESIST);
        base_stats[level].attack_damage += get_stat!(MEGA_GNAR_ATTACK_DAMAGE);
        level += 1;
    }
    base_stats
};

/// Simplified way to construct a new struct from the provided base stats and
/// current stats. Only structs with generic arguments `T` are accepted in this
/// macro
/// ```rs
/// let current_player_bonus_stats = bonus_stats!(
///     BasicStats::<f32>(champion_stats, current_player_base_stats) {
///         armor,
///         health,
///         attack_damage,
///         magic_resist,
///         mana
///     }
/// );
/// ```
#[macro_export]
macro_rules! bonus_stats {
    ($struct:ident::<$t:ty>($current_stats:expr, $base_stats:expr) { $($field:ident),*}) => {
        $struct::<$t> {
            $(
                $field: $current_stats.$field - $base_stats.$field,
            )*
        }
    };
}

pub use bonus_stats;

/// Checks if at least one of the provided [`ItemId`] in the array is in the
/// [`tutorlolv2_gen::ItemsBitSet`], similar to method [`core::iter::Iterator::any`]
pub const fn has_item<const N: usize>(origin: &ItemsBitSet, check_for: [ItemId; N]) -> bool {
    let mut i = 0;
    while i < N {
        if origin.contains(check_for[i] as usize) {
            return true;
        }
        i += 1;
    }
    false
}

/// Same as the method [`u8::clamp`] but with the `const` qualifier,
pub const fn const_clamp(value: u8, range: core::ops::RangeInclusive<u8>) -> usize {
    let min = *range.start();
    let max = *range.end();
    (if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }) as usize
}

/// Takes as parameters the enum [`ChampionId`] and the desired level of the current
/// champion and returns its base stats. If the level is higher than [`URF_MAX_LEVEL`],
/// the value is clamped to avoid panics
pub const fn get_base_stats(champion_id: ChampionId, level: u8) -> BasicStats<f32> {
    BASE_STATS[champion_id as usize][const_clamp(level, 1..=URF_MAX_LEVEL as u8) - 1]
}

pub const fn base_stats_sf32(
    champion_id: ChampionId,
    level: u8,
    is_mega_gnar: bool,
) -> SimpleStats<f32> {
    let BasicStats {
        health,
        armor,
        magic_resist,
        ..
    } = base_stats_bf32(champion_id, level, is_mega_gnar);
    SimpleStats {
        health,
        armor,
        magic_resist,
    }
}

pub const fn base_stats_bf32(
    champion_id: ChampionId,
    level: u8,
    is_mega_gnar: bool,
) -> BasicStats<f32> {
    match champion_id {
        ChampionId::Gnar if is_mega_gnar => {
            MEGA_GNAR_BASE_STATS[const_clamp(level, 1..=URF_MAX_LEVEL as u8) - 1]
        }
        _ => get_base_stats(champion_id, level),
    }
}

/// Receives the current player stats and the qualified dragons and returns a large array
/// of stats as if the player owned the qualified item, defined in the constant
/// [`SIMULATED_ITEMS_ENUM`]. The qualified items are defined by their tier, gold, and if
/// they're purchasable in the standard gamemode [`GameMap::SummonersRift`].
pub const fn get_simulated_stats(stats: &Stats<f32>, dragons: Dragons) -> [Stats<f32>; L_SIML] {
    let mut result = MaybeUninit::<[Stats<f32>; L_SIML]>::uninit();
    let result_ptr = result.as_mut_ptr();

    let mut i = 0;
    while i < SIMULATED_ITEMS_ENUM.len() {
        let mut new_stat = *stats;
        let cache = SIMULATED_ITEMS_ENUM[i].cache().stats;

        new_stat.armor_penetration_flat += cache.armor_penetration_flat;
        new_stat.magic_penetration_flat += cache.magic_penetration_flat;
        new_stat.ability_power += cache.ability_power;
        new_stat.attack_damage += cache.attack_damage;
        new_stat.magic_resist += cache.magic_resist;
        new_stat.attack_speed += cache.attack_speed;
        new_stat.crit_chance += cache.crit_chance;
        new_stat.crit_damage += cache.crit_damage;
        new_stat.health += cache.health;
        new_stat.armor += cache.armor;
        new_stat.mana += cache.mana;
        new_stat.armor_penetration_percent = RiotFormulas::percent_value(&[
            new_stat.armor_penetration_percent,
            cache.armor_penetration_percent,
        ]);
        new_stat.magic_penetration_percent = RiotFormulas::percent_value(&[
            new_stat.magic_penetration_percent,
            cache.magic_penetration_percent,
        ]);

        let earth_mod = get_earth_multiplier(dragons.ally_earth_dragons);
        let fire_mod = get_fire_multiplier(dragons.ally_fire_dragons);

        new_stat.ability_power *= fire_mod;
        new_stat.attack_damage *= fire_mod;
        new_stat.magic_resist *= earth_mod;
        new_stat.armor *= earth_mod;

        unsafe {
            core::ptr::addr_of_mut!((*result_ptr)[i]).write(new_stat);
        }

        i += 1;
    }

    unsafe { result.assume_init() }
}

/// Returns an instance [`DamageKind`] containing the closures and metadata of the runes.
/// Since the number of runes is unknown at compile time, those values are dynamically
/// allocated. This function does not evaluate any closures
pub fn get_runes_data(runes: &RunesBitSet, attack_type: AttackType) -> DamageKind<RuneId> {
    let count = runes.count() as usize;
    let mut metadata = Box::new_uninit_slice(count);
    let mut closures = Box::new_uninit_slice(count);
    unsafe {
        for (i, rune_offset) in runes.into_iter().enumerate() {
            let rune = RUNE_CACHE.get_unchecked(rune_offset);
            metadata.get_unchecked_mut(i).write(rune.metadata);
            closures.get_unchecked_mut(i).write(match attack_type {
                AttackType::Ranged => rune.ranged_damage,
                AttackType::Melee => rune.melee_damage,
            });
        }
        DamageKind {
            metadata: metadata.assume_init(),
            closures: closures.assume_init(),
        }
    }
}

/// Returns an instance [`DamageKind`] containing the closures and metadata of the items.
/// Since the number of items is unknown at compile time, those values are dynamically
/// allocated. This function does not evaluate any closures. Note that exclusively for
/// items, every [`ItemId`] has two closures for a single metadata object. In other words
/// ```rs
/// assert!(metadata.len() == closures.len() / 2)
/// ```
pub fn get_items_data(items: &ItemsBitSet, attack_type: AttackType) -> DamageKind<ItemId> {
    let count = items.count() as usize;

    let mut metadata = Box::new_uninit_slice(count);
    let mut closures = Box::new_uninit_slice(count << 1);

    unsafe {
        for (i, item_offset) in items.into_iter().enumerate() {
            let item = ITEM_CACHE.get_unchecked(item_offset);
            let slice = match attack_type {
                AttackType::Ranged => item.ranged_damages,
                AttackType::Melee => item.melee_damages,
            };

            metadata.get_unchecked_mut(i).write(item.metadata);
            let base = i << 1;
            closures.get_unchecked_mut(base).write(slice[0]);
            closures.get_unchecked_mut(base + 1).write(slice[1]);
        }

        DamageKind {
            metadata: metadata.assume_init(),
            closures: closures.assume_init(),
        }
    }
}

/// Converts a slice of [`RuneId`] into a [`RunesBitSet`], removing the ones that
/// do not deal any damage
pub const fn get_damaging_runes(input: &[RuneId]) -> RunesBitSet {
    let mut out = RunesBitSet::EMPTY;
    let mut i = 0;
    while i < input.len() {
        let rune = input[i] as usize;
        if DAMAGING_RUNES.contains(rune) {
            out.insert(rune);
        }
        i += 1;
    }
    out
}

/// Converts a slice of [`ItemId`] into a [`ItemsBitSet`], removing the ones that
/// do not deal any damage
pub const fn get_damaging_items(input: &[ItemId]) -> ItemsBitSet {
    let mut out = ItemsBitSet::EMPTY;
    let mut i = 0;
    while i < input.len() {
        let item = input[i] as usize;
        if DAMAGING_ITEMS.contains(item) {
            out.insert(item);
        }
        i += 1;
    }
    out
}

/// Mutates the variable `stats` and returns the bonus mana recovered from all items.
/// In general, information about the enemy's mana is useless, but there are some items
/// that increase their HP based on this stat. Because of that, having information about
/// the bonus mana allows a better estimate about the enemy's current HP
pub const fn get_enemy_current_stats(
    stats: &mut SimpleStats<f32>,
    items: &[ItemId],
    earth_dragons: u16,
) -> f32 {
    let mut bonus_mana = 0.0;

    let mut i = 0;
    while i < items.len() {
        let item = items[i].cache();
        stats.magic_resist += item.stats.magic_resist;
        stats.health += item.stats.health;
        stats.armor += item.stats.armor;
        bonus_mana += item.stats.mana;
        i += 1;
    }
    let dragon_mod = get_earth_multiplier(earth_dragons);
    stats.magic_resist *= dragon_mod;
    stats.armor *= dragon_mod;
    bonus_mana
}

/// Takes information about the current enemy and returns a struct that represents
/// all the useful information we should infer from the current enemy state. This
/// will be used to create a struct [`Ctx`] that will be used to evaluate
/// the closures of the current champion. Champion and item specific bonus stats
/// are applied in this function
pub const fn get_enemy_state(
    state: EnemyState,
    shred: ResistShred,
    accept_negatives: bool,
) -> EnemyFullState {
    let EnemyState {
        current_stats,
        base_stats,
        items,
        stacks,
        champion_id,
        earth_dragons,
        level,
        item_exceptions,
    } = state;
    let ResistShred {
        armor_penetration_flat,
        armor_penetration_percent,
        magic_penetration_flat,
        magic_penetration_percent,
    } = shred;

    let mut e_default_stats = base_stats;
    let bonus_mana = get_enemy_current_stats(&mut e_default_stats, items, earth_dragons);
    let mut e_modifiers = DamageModifiers::default();

    let mut i = 0;
    while i < item_exceptions.len() {
        let item_exception = item_exceptions[i];
        let stacks = item_exception.stacks();

        if let Some(item_id) = item_exception.get_item_id() {
            match item_id {
                ItemId::WintersApproach | ItemId::Fimbulwinter => {
                    e_default_stats.health += 0.15 * bonus_mana
                }
                ItemId::DragonheartU44 => {
                    let modifier = 1.0 + 0.04 * stacks as f32;
                    e_default_stats.health *= modifier;
                    e_default_stats.armor *= modifier;
                    e_default_stats.magic_resist *= modifier
                }
                ItemId::DemonKingsCrownU44 | ItemId::DemonKingsCrownU66 => {
                    let modifier = 1.0 + 0.01 * stacks as f32;
                    e_default_stats.health *= modifier;
                    e_default_stats.armor *= modifier;
                    e_default_stats.magic_resist *= modifier
                }
                ItemId::WarmogsArmor => e_default_stats.health *= 1.12,
                _ => {}
            }
        }
        i += 1;
    }

    match champion_id {
        ChampionId::Swain => {
            let stack_hp = 12 * stacks;
            e_default_stats.health += stack_hp as f32;
        }
        ChampionId::Chogath => {
            let stack_hp = stacks * 80
                + 40 * match level {
                    ..6 => 0,
                    6..11 => 1,
                    11..16 => 2,
                    16.. => 3,
                };
            e_default_stats.health += stack_hp as f32;
        }
        ChampionId::Sion => {
            e_default_stats.health += stacks as f32;
        }
        ChampionId::Kassadin => {
            // #![manual_impl]
            e_modifiers.magic_mod *= 0.9;
        }
        ChampionId::Ornn => {
            // Starts game with +10% armor/mr/hp already
            // After level 13, player will start upgrading items
            // At level 18, the maximum bonus must have been reached
            // For every upgrade, a +4% resist is applied.
            // #![manual_impl]
            let ornn_resist_multiplier = match level {
                ..13 => 1.1,
                13..18 => (level - 12) as f32 * 0.04,
                18.. => 1.3,
            };
            e_default_stats.armor *= ornn_resist_multiplier;
            e_default_stats.magic_resist *= ornn_resist_multiplier;
            e_default_stats.health *= ornn_resist_multiplier;
        }
        ChampionId::Malphite => {
            // W upgrade pattern for malphite by 06/07/2025
            // #![manual_impl]
            let malphite_resist_multiplier = match level {
                ..3 => 1.0,
                3..14 => 1.1,
                14 => 1.15,
                15..17 => 1.2,
                17 => 1.25,
                18.. => 1.3,
            };
            e_default_stats.armor *= malphite_resist_multiplier;
        }
        _ => {}
    }

    let e_current_stats = match current_stats {
        Some(s) => s,
        None => EnemyStats {
            armor: e_default_stats.armor,
            health: e_default_stats.health,
            magic_resist: e_default_stats.magic_resist,
            max_health: e_default_stats.health,
            missing_health: 1.0,
        },
    };

    let armor_values = RiotFormulas::real_resist(
        armor_penetration_percent,
        armor_penetration_flat,
        e_current_stats.armor,
        accept_negatives,
    );
    let magic_values = RiotFormulas::real_resist(
        magic_penetration_percent,
        magic_penetration_flat,
        e_current_stats.magic_resist,
        accept_negatives,
    );

    let e_bonus_stats = bonus_stats!(
        SimpleStats::<f32>(e_current_stats, base_stats) {
            armor,
            health,
            magic_resist
        }
    );

    let mut origin = ItemsBitSet::EMPTY;
    let mut i = 0;
    while i < items.len() {
        origin.insert(items[i] as usize);
        i += 1;
    }

    EnemyFullState {
        current_stats: e_current_stats,
        bonus_stats: e_bonus_stats,
        modifiers: e_modifiers,
        armor_values,
        magic_values,
        // #![manual_impl]
        steelcaps: has_item(&origin, [ItemId::PlatedSteelcaps, ItemId::ArmoredAdvance]),
        // #![manual_impl]
        rocksolid: has_item(
            &origin,
            [
                ItemId::RanduinsOmen,
                ItemId::FrozenHeart,
                ItemId::WardensMail,
            ],
        ),
        // #![manual_impl]
        randuin: has_item(&origin, [ItemId::RanduinsOmen]),
    }
}

/// Construct a new [`Ctx`] type that can be used to evaluate any champion's
/// closures and get their intermediary damage values, before applying the reductions
/// from armor and magic resist. See [`ConstClosure`] for more details about those
/// functions
pub const fn get_eval_ctx(self_state: &SelfState, e_state: &EnemyFullState) -> Ctx {
    let SelfState {
        stacks,
        ability_levels,
        current_stats:
            Stats {
                ability_power,
                armor,
                armor_penetration_flat,
                armor_penetration_percent,
                attack_damage,
                attack_speed,
                crit_chance,
                crit_damage,
                current_health,
                magic_penetration_flat,
                magic_penetration_percent,
                magic_resist,
                health: max_health,
                mana: max_mana,
                current_mana,
                ..
            },
        bonus_stats:
            BasicStats {
                attack_damage: bonus_ad,
                armor: bonus_armor,
                magic_resist: bonus_magic_resist,
                health: bonus_health,
                mana: bonus_mana,
            },
        base_stats:
            BasicStats {
                armor: base_armor,
                health: base_health,
                attack_damage: base_ad,
                magic_resist: base_magic_resist,
                mana: base_mana,
            },
        level,
        adaptative_type,
    } = *self_state;
    let EnemyFullState {
        current_stats:
            EnemyStats {
                armor: enemy_armor,
                health: enemy_health,
                magic_resist: enemy_magic_resist,
                max_health: enemy_max_health,
                missing_health: enemy_missing_health,
            },
        bonus_stats:
            SimpleStats {
                health: enemy_bonus_health,
                armor: enemy_bonus_armor,
                magic_resist: enemy_bonus_magic_resist,
            },
        armor_values,
        magic_values,
        steelcaps,
        rocksolid,
        randuin,
        ..
    } = *e_state;
    Ctx {
        q_level: ability_levels.q as _,
        w_level: ability_levels.w as _,
        e_level: ability_levels.e as _,
        r_level: ability_levels.r as _,
        level: level as _,
        physical_multiplier: armor_values.modifier,
        magic_multiplier: magic_values.modifier,
        enemy_bonus_health,
        enemy_bonus_armor,
        enemy_bonus_magic_resist,
        enemy_armor,
        enemy_health,
        enemy_max_health,
        enemy_missing_health,
        enemy_magic_resist,
        base_health,
        base_ad,
        base_armor,
        base_magic_resist,
        base_mana,
        bonus_ad,
        bonus_armor,
        bonus_magic_resist,
        bonus_health,
        bonus_mana,
        bonus_move_speed: 1.0,
        armor_penetration_flat,
        armor_penetration_percent,
        magic_penetration_flat,
        magic_penetration_percent,
        max_mana,
        current_mana,
        max_health,
        current_health,
        armor,
        magic_resist,
        crit_chance,
        crit_damage,
        attack_speed,
        missing_health: 1.0 - (current_health / max_health.max(1.0)),
        ability_power,
        attack_damage,
        adaptative_damage: match adaptative_type {
            AdaptativeType::Physical => armor_values.modifier,
            AdaptativeType::Magic => magic_values.modifier,
        },
        steelcaps_effect: match steelcaps {
            true => STEEL_CAPS_PROTECTION,
            false => 1.0,
        },
        randuin_effect: match randuin {
            true => RANDUIN_CRIT_PROTECTION,
            false => 1.0,
        },
        rocksolid_effect: match rocksolid {
            true => ROCKSOLID_PROTECTION,
            false => 1.0,
        },
        stacks,
    }
}

pub const fn get_stacks(champion_id: ChampionId, game_time: f32) -> f32 {
    const AVERAGE_GAME_TIME: u32 = 60 * 30;
    pub const fn time(game_time: f32, stacks_at_30m: u32) -> f32 {
        (AVERAGE_GAME_TIME / stacks_at_30m) as f32 / game_time
    }
    match champion_id {
        ChampionId::Chogath => time(game_time, 12),
        ChampionId::Nasus => time(game_time, 450),
        ChampionId::Smolder => time(game_time, 120),
        ChampionId::AurelionSol => time(game_time, 190),
        ChampionId::Thresh => time(game_time, 150),
        ChampionId::Kindred => time(game_time, 5),
        _ => 1.0,
    }
}

/// Inserts ability modifier buffs or debuffs for each individual ability, based on their
/// letter discriminant: `P`, `Q`, `W`, `E`, `R`.
pub const fn ability_id_mod(
    ability_id: AbilityId,
    damage_type: DamageType,
    modifiers: Modifiers,
) -> f32 {
    let Modifiers { damages, abilities } = modifiers;
    let mut modifier = damages.modifier(damage_type);

    if let Some((v, modf)) = match ability_id {
        AbilityId::Q(v) => Some((v, abilities.q)),
        AbilityId::W(v) => Some((v, abilities.w)),
        AbilityId::E(v) => Some((v, abilities.e)),
        AbilityId::R(v) => Some((v, abilities.r)),
        _ => None,
    } && v as u8 <= AbilityName::Mega as u8
    {
        modifier *= modf;
    }
    modifier
}

/// Evaluates the damage of all provided metadata of [`AbilityId`]. This function
/// already multiplies the final damage result by the appropriate armor, or magic
/// resist multiplier of the enemy, and considers global and local damage modifiers
/// This function will cause `Undefined Behavior` if the length of `closures` and
/// `metadata` are not equal. See similar functions [`item_id_eval_damage`] and
/// [`rune_id_eval_damage`]
pub fn ability_id_eval_damage(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    metadata: &[TypeMetadata<AbilityId>],
    closures: &[ConstClosure],
    modifiers: Modifiers,
) -> Box<[i32]> {
    let len = metadata.len();
    debug_assert_eq!(len, closures.len());

    (0..len)
        .map(|i| {
            let TypeMetadata {
                kind,
                damage_type,
                attributes,
            } = metadata[i];
            let closure = unsafe { closures.get_unchecked(i) };
            let modifier = ability_id_mod(kind, damage_type, modifiers);
            let damage = (modifier * closure(ctx)) as i32;
            onhit.inc_attr(attributes, damage);
            damage
        })
        .collect()
}

/// Evaluates the damages of all requested items. This function causes
/// `Undefined Behavior` if the length of `closures` is not twice as much as `metadata`
/// or panic in debug mode. See similar function [`ability_id_eval_damage`] and
/// [`rune_id_eval_damage`]
pub fn item_id_eval_damage(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    metadata: &[TypeMetadata<ItemId>],
    closures: &[ConstClosure],
    modifiers: Modifiers,
) -> Box<[i32]> {
    let out_len = closures.len();
    debug_assert_eq!(out_len, metadata.len() << 1);
    let mut result = Box::<[i32]>::new_uninit_slice(out_len);

    let mut meta_index = 0usize;
    let mut out_index = 0usize;

    while meta_index < metadata.len() {
        let TypeMetadata {
            damage_type,
            attributes,
            ..
        } = unsafe { metadata.get_unchecked(meta_index) };
        let modifier = modifiers.damages.modifier(*damage_type);
        let mut j = 0;
        while j < 2 {
            let closure = unsafe { closures.get_unchecked((meta_index << 1) + j) };
            let damage = (modifier * closure(ctx)) as i32;
            onhit.inc_attr(*attributes, damage);
            unsafe {
                result.get_unchecked_mut(out_index).write(damage);
            }
            out_index += 1;
            j += 1;
        }
        meta_index += 1;
    }

    debug_assert_eq!(out_index, out_len);
    unsafe { result.assume_init() }
}

/// Evaluates the damages of all runes that deal damage, owned by the current player.
/// This function causes `Undefined Behavior` if the length of `closures` is not equal
/// to the length of `metadata`. See similar function [`ability_id_eval_damage`] and
/// [`item_id_eval_damage`]
pub fn rune_id_eval_damage(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    metadata: &[TypeMetadata<RuneId>],
    closures: &[ConstClosure],
    modifiers: Modifiers,
) -> Box<[i32]> {
    let len = metadata.len();
    debug_assert_eq!(len, closures.len());

    (0..len)
        .map(|i| {
            let TypeMetadata {
                damage_type,
                attributes,
                ..
            } = unsafe { metadata.get_unchecked(i) };
            let modifier = modifiers.damages.modifier(*damage_type);
            let closure = unsafe { closures.get_unchecked(i) };
            let damage = (modifier * closure(ctx)) as i32;
            onhit.inc_attr(*attributes, damage);
            damage
        })
        .collect()
}

/// Evaluates the damage of basic attacks, onhit damages and critical strikes
pub const fn eval_attacks(ctx: &Ctx, mut onhit_damage: RangeDamage, physical_mod: f32) -> Attacks {
    let basic_attack = ctx.attack_damage * physical_mod;
    let critical_strike = (basic_attack * ctx.crit_damage / 100.0) as i32;
    let basic_attack = basic_attack as i32;

    onhit_damage.minimum_damage += basic_attack;
    onhit_damage.maximum_damage += critical_strike;

    Attacks {
        basic_attack,
        critical_strike,
        onhit_damage,
    }
}

/// Confirms that every single metadata array have the same length as the closures array,
/// for every champion. Also, for every item, the melee and range closures should also have
/// the same number of elements inside. If this is not done, some functions in this module
/// will panic or cause undefined behavior
const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion_id = ChampionId::from_usize(i).unwrap();
        let CachedChampion {
            metadata, closures, ..
        } = champion_id.cache();
        assert!(metadata.len() == closures.len());
        i += 1;
    }
    let mut j = 0;
    while j < ItemId::VARIANTS {
        let item_id = ItemId::from_usize(j).unwrap();
        let CachedItem {
            melee_damages: melee_closure,
            ranged_damages: range_closure,
            ..
        } = item_id.cache();
        assert!(melee_closure.len() == range_closure.len());
        j += 1;
    }
};

/// Constructs a new [`Damages`] struct that holds all the damage values against some entity
/// that could be calculated. This function will cause undefined behavior if any
/// metadata of closures vectors do not have the same length
pub fn get_damages(ctx: Ctx, data: &DamageEvalData, modifiers: Modifiers) -> Damages {
    let mut onhit = RangeDamage::default();

    let abilities = ability_id_eval_damage(
        &ctx,
        &mut onhit,
        data.abilities.metadata,
        data.abilities.closures,
        modifiers,
    );
    let items = item_id_eval_damage(
        &ctx,
        &mut onhit,
        &data.items.metadata,
        &data.items.closures,
        modifiers,
    );
    let runes = rune_id_eval_damage(
        &ctx,
        &mut onhit,
        &data.runes.metadata,
        &data.runes.closures,
        modifiers,
    );
    let attacks = eval_attacks(&ctx, onhit, modifiers.damages.physical_mod);

    Damages {
        abilities,
        items,
        runes,
        attacks,
        ctx,
    }
}

/// Returns the damage against monsters with different amounts of armor and magic resists.
/// We assume no runes can damage jungle monsters, which is why they're not included in the
/// [`MonsterDamage`] struct
pub fn get_monster_damages(
    self_state: &SelfState,
    eval_data: &DamageEvalData,
    shred: ResistShred,
) -> [Damages; L_MSTR] {
    core::array::from_fn(|i| {
        let (armor, magic_resist) = MONSTER_RESISTS[i];
        let full_state = get_enemy_state(
            EnemyState {
                base_stats: SimpleStats {
                    armor,
                    health: 1000.0,
                    magic_resist,
                },
                ..Default::default()
            },
            shred,
            true,
        );
        let ctx = get_eval_ctx(self_state, &full_state);
        let modifiers = Modifiers::new(&ctx);
        get_damages(ctx, eval_data, modifiers)
    })
}

/// Returns an array with the damage against turrets of different plates.
/// The array has 6 ([`L_TWRD`]) elements because a tower can have zero,
/// or up to 5 plates
pub const fn get_tower_damages(
    adaptative_type: AdaptativeType,
    base_attack_damage: f32,
    bonus_attack_damage: f32,
    ability_power: f32,
    shred: ResistShred,
) -> [i32; L_TWRD] {
    let mut tower_damages = MaybeUninit::<[i32; L_TWRD]>::uninit();
    let tower_ptr = tower_damages.as_mut_ptr();
    let mut i = 0;

    let (pen_percent, pen_flat) = match adaptative_type {
        AdaptativeType::Physical => (
            shred.armor_penetration_percent,
            shred.armor_penetration_flat,
        ),
        AdaptativeType::Magic => (
            shred.magic_penetration_percent,
            shred.magic_penetration_flat,
        ),
    };

    while i < L_TWRD {
        let damage = RiotFormulas::tower_damage(
            i as _,
            base_attack_damage,
            bonus_attack_damage,
            ability_power,
            pen_percent,
            pen_flat,
        );
        unsafe {
            core::ptr::addr_of_mut!((*tower_ptr)[i]).write(damage);
        }
        i += 1;
    }
    unsafe { tower_damages.assume_init() }
}
