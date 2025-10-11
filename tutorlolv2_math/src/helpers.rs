use super::model::*;
use crate::{L_ITEM, L_RUNE, L_SIML, NUMBER_OF_CHAMPIONS, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::*;

pub const AXIOM_ARCANIST_BONUS_DAMAGE: f32 = 1.12;
pub const COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE: f32 = 1.08;
/// By 06/07/2025 Earth dragons give +5% resists
/// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
/// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;
pub const LAST_STAND_CLOSURE: fn(f32) -> f32 =
    |missing_health| 1.0 + (0.05 + 0.2 * (missing_health - 0.4)).clamp(0.0, 0.11);
pub const GET_FIRE_MULTIPLIER: fn(u16) -> f32 = |x| 1.0 + x as f32 * FIRE_DRAGON_MULTIPLIER;
pub const GET_EARTH_MULTIPLIER: fn(u16) -> f32 = |x| 1.0 + x as f32 * EARTH_DRAGON_MULTIPLIER;
pub const URF_MAX_LEVEL: usize = 30;
/// Ordered as: health, armor, magic_resist, attack_damage, mana
pub static BASE_STATS: [[[f32; 5]; URF_MAX_LEVEL]; NUMBER_OF_CHAMPIONS] = {
    let mut base_stats = [[[0.0; 5]; URF_MAX_LEVEL]; NUMBER_OF_CHAMPIONS];
    let mut champion_index = 0;
    while champion_index < NUMBER_OF_CHAMPIONS {
        let mut level = 0;
        while level < URF_MAX_LEVEL {
            let stats = &INTERNAL_CHAMPIONS[champion_index].stats;
            let growth_factor = RiotFormulas::growth(level as u8 + 1);
            macro_rules! get_stat {
                ($field:ident) => {
                    RiotFormulas::stat_growth(
                        stats.$field.flat,
                        stats.$field.per_level,
                        growth_factor,
                    )
                };
            }
            let health = get_stat!(health);
            let armor = get_stat!(armor);
            let magic_resist = get_stat!(magic_resist);
            let attack_damage = get_stat!(attack_damage);
            let mana = get_stat!(mana);
            base_stats[champion_index][level] = [health, armor, magic_resist, attack_damage, mana];
            level += 1;
        }
        champion_index += 1;
    }
    base_stats
};
pub const MEGA_GNAR_HEALTH: CachedChampionStatsMap = CachedChampionStatsMap {
    flat: 100.0,
    per_level: 43.0,
};
pub const MEGA_GNAR_ARMOR: CachedChampionStatsMap = CachedChampionStatsMap {
    flat: 3.5,
    per_level: 3.0,
};
pub const MEGA_GNAR_MAGIC_RESIST: CachedChampionStatsMap = CachedChampionStatsMap {
    flat: 3.5,
    per_level: 3.5,
};
pub const MEGA_GNAR_ATTACK_DAMAGE: CachedChampionStatsMap = CachedChampionStatsMap {
    flat: 6.0,
    per_level: 2.5,
};
pub static MEGA_GNAR_BASE_STATS: [[f32; 5]; URF_MAX_LEVEL] = {
    let mut base_stats = BASE_STATS[ChampionId::Gnar as usize];
    let mut level = 0;
    while level < URF_MAX_LEVEL {
        let growth_factor = RiotFormulas::growth(level as u8 + 1);
        macro_rules! get_stat {
            ($field:ident) => {
                RiotFormulas::stat_growth($field.flat, $field.per_level, growth_factor)
            };
        }
        base_stats[level][0] += get_stat!(MEGA_GNAR_HEALTH);
        base_stats[level][1] += get_stat!(MEGA_GNAR_ARMOR);
        base_stats[level][2] += get_stat!(MEGA_GNAR_MAGIC_RESIST);
        base_stats[level][3] += get_stat!(MEGA_GNAR_ATTACK_DAMAGE);
        base_stats[level][4] = 0.0;
        level += 1;
    }
    base_stats
};

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

/// Level must be in range 1..=30. Returns the index in BASE_STATS level array.
pub const fn clamp_level(level: u8) -> usize {
    let min = 1;
    let max = 30;
    ((if level < min {
        min
    } else if level > max {
        max
    } else {
        level
    }) - 1) as usize
}

pub const fn get_base_stats(champion_id: ChampionId, level: u8) -> &'static [f32; 5] {
    &BASE_STATS[champion_id as usize][clamp_level(level)]
}

impl BasicStats<f32> {
    #[inline]
    pub const fn from_slice(stats: &[f32; 5]) -> Self {
        let [health, armor, magic_resist, attack_damage, mana] = *stats;
        Self {
            health,
            armor,
            magic_resist,
            attack_damage,
            mana,
        }
    }
}

impl SimpleStats<f32> {
    #[inline]
    pub const fn from_slice(stats: &[f32; 5]) -> Self {
        let [health, armor, magic_resist, _, _] = *stats;
        Self {
            health,
            armor,
            magic_resist,
        }
    }
}

macro_rules! impl_base_stats {
    ($struct:ident) => {
        impl $struct<f32> {
            pub const fn base_stats(
                champion_id: ChampionId,
                level: u8,
                is_mega_gnar: bool,
            ) -> Self {
                if is_mega_gnar {
                    Self::from_slice(&MEGA_GNAR_BASE_STATS[clamp_level(level)])
                } else {
                    Self::from_slice(get_base_stats(champion_id, level))
                }
            }
        }
    };
}

impl_base_stats!(SimpleStats);
impl_base_stats!(BasicStats);

pub fn has_item<const N: usize>(origin: &SetU32, check_for: [ItemId; N]) -> bool {
    check_for
        .into_iter()
        .any(|item_id| origin.contains(item_id as u32))
}

pub fn get_simulated_stats(stats: &Stats<f32>, dragons: Dragons) -> [Stats<f32>; L_SIML] {
    let mut result = MaybeUninit::<[Stats<f32>; L_SIML]>::uninit();
    let result_ptr = result.as_mut_ptr();

    for (i, item_offset) in SIMULATED_ITEMS_ENUM.into_iter().enumerate() {
        let item_cache = unsafe { INTERNAL_ITEMS.get_unchecked(item_offset as usize) };
        let mut new_stat = *stats;

        macro_rules! add_stat {
            ($field:ident) => {
                new_stat.$field += item_cache.stats.$field;
            };
            (@$field:ident) => {
                new_stat.$field = RiotFormulas::percent_value(&[new_stat.$field, stats.$field]);
            };
        }

        add_stat!(mana);
        add_stat!(health);
        add_stat!(magic_resist);
        add_stat!(crit_chance);
        add_stat!(crit_damage);
        add_stat!(ability_power);
        add_stat!(attack_damage);
        add_stat!(armor);
        add_stat!(attack_speed);
        add_stat!(armor_penetration_flat);
        add_stat!(magic_penetration_flat);
        add_stat!(@armor_penetration_percent);
        add_stat!(@magic_penetration_percent);

        let fire_mod = GET_FIRE_MULTIPLIER(dragons.ally_fire_dragons);
        let earth_mod = GET_EARTH_MULTIPLIER(dragons.ally_earth_dragons);

        new_stat.ability_power *= fire_mod;
        new_stat.attack_damage *= fire_mod;
        new_stat.magic_resist *= earth_mod;
        new_stat.armor *= earth_mod;

        unsafe {
            core::ptr::addr_of_mut!((*result_ptr)[i]).write(new_stat);
        }
    }

    unsafe { result.assume_init() }
}

pub fn get_runes_data(runes: &SetU32, attack_type: AttackType) -> DamageKind<L_RUNE, RuneId> {
    let mut metadata = SmallVec::with_capacity(runes.len());
    let mut closures = SmallVec::with_capacity(runes.len());
    for rune_number in runes.iter() {
        let rune = unsafe { INTERNAL_RUNES.get_unchecked(rune_number as usize) };
        closures.push(match attack_type {
            AttackType::Ranged => rune.range_closure,
            AttackType::Melee => rune.melee_closure,
        });
        metadata.push(rune.metadata);
    }
    DamageKind { metadata, closures }
}

pub fn get_items_data(items: &SetU32, attack_type: AttackType) -> DamageKind<L_ITEM, ItemId> {
    let mut metadata = SmallVec::with_capacity(items.len());
    let mut closures = SmallVec::with_capacity(items.len());
    for item_number in items.iter() {
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(item_number as usize) };
        closures.push(match attack_type {
            AttackType::Ranged => item.range_closure,
            AttackType::Melee => item.melee_closure,
        });
        metadata.push(item.metadata);
    }
    DamageKind { metadata, closures }
}

pub fn runes_slice_to_set_u32(input: &[RuneId]) -> SetU32 {
    input
        .iter()
        .filter_map(|rune| {
            DAMAGING_RUNES
                .contains(&rune.to_riot_id())
                .then_some(*rune as u32)
        })
        .collect::<SetU32>()
}

pub fn items_slice_to_set_u32(input: &[ItemId]) -> SetU32 {
    input
        .iter()
        .filter_map(|item| {
            DAMAGING_ITEMS
                .contains(&item.to_riot_id())
                .then_some(*item as u32)
        })
        .collect::<SetU32>()
}

pub fn get_enemy_current_stats(
    stats: &mut SimpleStats<f32>,
    items: &SetU32,
    earth_dragons: u16,
) -> f32 {
    let mut bonus_mana = 0.0;
    for item_id in items.iter() {
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(item_id as usize) };
        macro_rules! add_value {
            ($field:ident) => {
                stats.$field += item.stats.$field;
            };
        }
        add_value!(health);
        add_value!(armor);
        add_value!(magic_resist);
        bonus_mana += item.stats.mana;
    }
    let dragon_mod = GET_EARTH_MULTIPLIER(earth_dragons);
    stats.armor *= dragon_mod;
    stats.magic_resist *= dragon_mod;
    bonus_mana
}

pub fn get_enemy_state(
    state: EnemyState,
    shred: ResistShred,
    accept_negatives: bool,
) -> EnemyFullState {
    let mut e_current_stats = state.base_stats;
    let e_items = &state.items;
    let stacks = state.stacks as f32;

    let bonus_mana = get_enemy_current_stats(&mut e_current_stats, &e_items, state.earth_dragons);

    let mut e_modifiers = DamageModifiers::default();

    for item_exception in state.item_exceptions.iter() {
        let stacks = item_exception.stacks();

        if let Some(item_id) = item_exception.get_item_id() {
            match item_id {
                ItemId::WintersApproach | ItemId::Fimbulwinter => {
                    e_current_stats.health += 0.15 * bonus_mana
                }
                ItemId::Dragonheart => {
                    let modifier = 1.0 + 0.04 * stacks as f32;
                    e_current_stats.health *= modifier;
                    e_current_stats.armor *= modifier;
                    e_current_stats.magic_resist *= modifier
                }
                ItemId::DemonKingsCrown => {
                    let modifier = 1.0 + 0.01 * stacks as f32;
                    e_current_stats.health *= modifier;
                    e_current_stats.armor *= modifier;
                    e_current_stats.magic_resist *= modifier
                }
                ItemId::WarmogsArmor => e_current_stats.health *= 1.12,
                _ => {}
            }
        }
    }

    match state.champion_id {
        ChampionId::Swain => {
            let stack_hp = 12.0 * stacks;
            e_current_stats.health += stack_hp;
        }
        ChampionId::Chogath => {
            let stack_hp = stacks * 80.0
                + 40.0
                    * match state.level {
                        ..6 => 0.0,
                        6..11 => 1.0,
                        11..16 => 2.0,
                        16.. => 3.0,
                    };
            e_current_stats.health += stack_hp;
        }
        ChampionId::Sion => {
            e_current_stats.health += stacks;
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
            let ornn_resist_multiplier = match state.level {
                ..13 => 1.1,
                13..18 => (state.level - 12) as f32 * 0.04,
                18.. => 1.3,
            };
            macro_rules! assign_value {
                ($field:ident) => {
                    e_current_stats.$field *= ornn_resist_multiplier;
                };
            }
            assign_value!(armor);
            assign_value!(magic_resist);
            assign_value!(health);
        }
        ChampionId::Malphite => {
            // W upgrade pattern for malphite by 06/07/2025
            // #![manual_impl]
            let malphite_resist_multiplier = match state.level {
                ..3 => 1.0,
                3..14 => 1.1,
                14 => 1.15,
                15..17 => 1.2,
                17 => 1.25,
                18.. => 1.3,
            };
            e_current_stats.armor *= malphite_resist_multiplier;
        }
        _ => {}
    }

    let armor_values = RiotFormulas::real_resist(
        shred.armor_penetration_percent,
        shred.armor_penetration_flat,
        e_current_stats.armor,
        accept_negatives,
    );
    let magic_values = RiotFormulas::real_resist(
        shred.magic_penetration_percent,
        shred.magic_penetration_flat,
        e_current_stats.magic_resist,
        accept_negatives,
    );

    let e_bonus_stats = bonus_stats!(
        SimpleStats::<f32>(e_current_stats, state.base_stats) {
            armor,
            health,
            magic_resist
        }
    );

    EnemyFullState {
        current_stats: e_current_stats,
        bonus_stats: e_bonus_stats,
        modifiers: e_modifiers,
        armor_values,
        magic_values,
        // #![manual_impl]
        steelcaps: has_item(e_items, [ItemId::PlatedSteelcaps, ItemId::ArmoredAdvance]),
        // #![manual_impl]
        rocksolid: has_item(
            e_items,
            [
                ItemId::RanduinsOmen,
                ItemId::FrozenHeart,
                ItemId::WardensMail,
            ],
        ),
        // #![manual_impl]
        randuin: has_item(e_items, [ItemId::RanduinsOmen]),
    }
}

pub const fn get_eval_ctx(self_state: &SelfState, e_state: &EnemyFullState) -> EvalContext {
    EvalContext {
        q_level: self_state.ability_levels.q,
        w_level: self_state.ability_levels.w,
        e_level: self_state.ability_levels.e,
        r_level: self_state.ability_levels.r,
        level: self_state.level as f32,
        chogath_stacks: 1.0,
        veigar_stacks: 1.0,
        nasus_stacks: 1.0,
        smolder_stacks: 1.0,
        aurelion_sol_stacks: 1.0,
        thresh_stacks: 1.0,
        kindred_stacks: 1.0,
        belveth_stacks: 1.0,
        adaptative_damage: match RiotFormulas::adaptative_type(
            self_state.bonus_stats.attack_damage,
            self_state.current_stats.ability_power,
        ) {
            AdaptativeType::Physical => e_state.armor_values.modifier,
            AdaptativeType::Magic => e_state.magic_values.modifier,
        },
        physical_multiplier: e_state.armor_values.modifier,
        magic_multiplier: e_state.magic_values.modifier,
        // #![manual_impl]
        steelcaps_effect: if e_state.steelcaps { 0.88 } else { 1.0 },
        // #![manual_impl]
        randuin_effect: if e_state.randuin { 0.7 } else { 1.0 },
        // #![manual_impl]
        rocksolid_effect: if e_state.rocksolid { 0.8 } else { 1.0 },
        enemy_bonus_health: e_state.bonus_stats.health,
        enemy_armor: e_state.current_stats.armor,
        enemy_max_health: e_state.current_stats.health,
        enemy_health: e_state.current_stats.health,
        enemy_current_health: e_state.current_stats.health,
        enemy_missing_health: e_state.current_stats.health,
        enemy_magic_resist: e_state.current_stats.magic_resist,
        base_health: self_state.base_stats.health,
        base_ad: self_state.base_stats.attack_damage,
        base_armor: self_state.base_stats.armor,
        base_magic_resist: self_state.base_stats.magic_resist,
        base_mana: self_state.base_stats.mana,
        bonus_ad: self_state.bonus_stats.attack_damage,
        bonus_armor: self_state.bonus_stats.armor,
        bonus_magic_resist: self_state.bonus_stats.magic_resist,
        bonus_health: self_state.bonus_stats.health,
        bonus_mana: self_state.bonus_stats.mana,
        // #![unsupported]
        bonus_move_speed: 1.0,
        armor_penetration_flat: self_state.current_stats.armor_penetration_flat,
        armor_penetration_percent: self_state.current_stats.armor_penetration_percent,
        magic_penetration_flat: self_state.current_stats.magic_penetration_flat,
        magic_penetration_percent: self_state.current_stats.magic_penetration_percent,
        max_mana: self_state.current_stats.mana,
        current_mana: self_state.current_stats.current_mana,
        max_health: self_state.current_stats.health,
        current_health: self_state.current_stats.current_health,
        armor: self_state.current_stats.armor,
        magic_resist: self_state.current_stats.magic_resist,
        crit_chance: self_state.current_stats.crit_chance,
        crit_damage: self_state.current_stats.crit_damage,
        attack_speed: self_state.current_stats.attack_speed,
        missing_health: 1.0
            - (self_state.current_stats.current_health / self_state.current_stats.health.max(1.0)),
        ap: self_state.current_stats.ability_power,
        ad: self_state.current_stats.attack_damage,
    }
}

pub trait IsAbility {
    fn apply_modifiers(&self, _: &mut f32, _: &AbilityModifiers) {}
}

impl IsAbility for ItemId {}
impl IsAbility for RuneId {}
impl IsAbility for AbilityLike {
    fn apply_modifiers(&self, modifier: &mut f32, ability_modifiers: &AbilityModifiers) {
        let mut modify = |ability_name: AbilityName, value: f32| {
            if ability_name as u8 <= AbilityName::_8Min as u8 {
                *modifier *= value
            }
        };
        match self {
            Self::Q(v) => modify(*v, ability_modifiers.q),
            Self::W(v) => modify(*v, ability_modifiers.w),
            Self::E(v) => modify(*v, ability_modifiers.e),
            Self::R(v) => modify(*v, ability_modifiers.r),
            _ => {}
        }
    }
}

pub fn eval_damage<const N: usize, T: IsAbility + 'static>(
    ctx: &EvalContext,
    onhit: &mut RangeDamage,
    metadata: &[TypeMetadata<T>],
    closures: &[DamageClosures],
    modifiers: Modifiers,
) -> SmallVec<[RangeDamage; N]> {
    let len = metadata.len();
    let mut result = SmallVec::with_capacity(len);
    for i in 0..len {
        let closure = unsafe { closures.get_unchecked(i) };
        let metadata = unsafe { metadata.get_unchecked(i) };
        let damage_type = metadata.damage_type;
        let attributes = metadata.attributes;

        let mut modifier = match damage_type {
            DamageType::Physical => modifiers.damages.physical_mod,
            DamageType::Magic => modifiers.damages.magic_mod,
            DamageType::True => modifiers.damages.true_mod,
            _ => 1.0,
        } * modifiers.damages.global_mod;

        metadata
            .kind
            .apply_modifiers(&mut modifier, &modifiers.abilities);

        let minimum_damage = (modifier * (closure.minimum_damage)(ctx)) as i32;
        let maximum_damage = (modifier * (closure.maximum_damage)(ctx)) as i32;

        let sum = minimum_damage + minimum_damage;
        match attributes {
            Attrs::OnhitMin => {
                onhit.minimum_damage += sum;
            }
            Attrs::OnhitMax => {
                onhit.maximum_damage += sum;
            }
            Attrs::Onhit => {
                onhit.minimum_damage += sum;
                onhit.maximum_damage += sum;
            }
            _ => {}
        };

        result.push(RangeDamage {
            minimum_damage,
            maximum_damage,
        });
    }
    result
}

pub fn eval_attacks(ctx: &EvalContext, mut onhit_damage: RangeDamage) -> Attacks {
    let basic_attack_damage = (BASIC_ATTACK.minimum_damage)(ctx) as i32;
    let critical_strike_damage = (CRITICAL_STRIKE.minimum_damage)(ctx) as i32;

    onhit_damage.minimum_damage += basic_attack_damage;
    onhit_damage.maximum_damage += critical_strike_damage;

    Attacks {
        basic_attack: RangeDamage {
            minimum_damage: basic_attack_damage,
            maximum_damage: 0,
        },
        critical_strike: RangeDamage {
            minimum_damage: 0,
            maximum_damage: critical_strike_damage,
        },
        onhit_damage,
    }
}

pub fn get_damages(eval_ctx: &EvalContext, data: &DamageEvalData, modifiers: Modifiers) -> Damages {
    let mut onhit = RangeDamage::default();

    macro_rules! eval_nonempty {
        ($name:ident) => {
            if data.$name.closures.is_empty() {
                SmallVec::new()
            } else {
                eval_damage(
                    &eval_ctx,
                    &mut onhit,
                    &data.$name.metadata,
                    &data.$name.closures,
                    modifiers,
                )
            }
        };
    }

    let abilities = eval_nonempty!(abilities);
    let items = eval_nonempty!(items);
    let runes = eval_nonempty!(runes);
    let attacks = eval_attacks(&eval_ctx, onhit);

    Damages {
        abilities,
        items,
        runes,
        attacks,
    }
}
