use super::model::*;
use crate::{L_ITEM, L_RUNE, L_SIML, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tutorlolv2_gen::*;
use tutorlolv2_types::{AbilityLike, AbilityName};

pub const AXIOM_ARCANIST_BONUS_DAMAGE: f32 = 1.12;
pub const COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE: f32 = 1.08;
/// By 06/07/2025 Earth dragons give +5% resists
/// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
/// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;
pub const URF_MAX_LEVEL: usize = 30;

pub const fn get_last_stand(missing_health: f32) -> f32 {
    1.0 + (0.05 + 0.2 * (missing_health - 0.4)).clamp(0.0, 0.11)
}

pub const fn get_earth_multiplier(x: u16) -> f32 {
    1.0 + x as f32 * EARTH_DRAGON_MULTIPLIER
}

pub const fn get_fire_multiplier(x: u16) -> f32 {
    1.0 + x as f32 * FIRE_DRAGON_MULTIPLIER
}

/// Ordered as: health, armor, magic_resist, attack_damage, mana
pub static BASE_STATS: [[BasicStats<f32>; URF_MAX_LEVEL]; NUMBER_OF_CHAMPIONS] = {
    let mut base_stats = [[BasicStats::default(); URF_MAX_LEVEL]; NUMBER_OF_CHAMPIONS];
    let mut champion_index = 0;
    while champion_index < NUMBER_OF_CHAMPIONS {
        let mut level = 0;
        while level < URF_MAX_LEVEL {
            let stats = &INTERNAL_CHAMPIONS[champion_index].stats;
            let growth_factor = RiotFormulas::growth(level as u8 + 1);
            macro_rules! mount_basic_stats {
                ($($field:ident),*) => {
                    BasicStats {
                        $(
                            $field: RiotFormulas::stat_growth(
                                stats.$field.flat,
                                stats.$field.per_level,
                                growth_factor,
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

pub static MEGA_GNAR_BASE_STATS: [BasicStats<f32>; URF_MAX_LEVEL] = {
    let mut base_stats = BASE_STATS[ChampionId::Gnar as usize];
    let mut level = 0;
    while level < URF_MAX_LEVEL {
        let growth_factor = RiotFormulas::growth(level as u8 + 1);

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
                RiotFormulas::stat_growth($field.flat, $field.per_level, growth_factor)
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

pub const fn has_item<const N: usize>(origin: &BitArray, check_for: [ItemId; N]) -> bool {
    let mut i = 0;
    while i < N {
        if origin.contains(check_for[i] as usize) {
            return true;
        }
        i += 1;
    }
    false
}

pub const fn const_clamp(value: u8, range: std::ops::RangeInclusive<u8>) -> usize {
    let min = *range.start();
    let max = *range.end();
    ((if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }) - 1) as usize
}

pub const fn get_base_stats(champion_id: ChampionId, level: u8) -> BasicStats<f32> {
    BASE_STATS[champion_id as usize][const_clamp(level, 1..=URF_MAX_LEVEL as u8)]
}

impl SimpleStats<f32> {
    pub const fn default() -> Self {
        Self {
            health: 0.0,
            armor: 0.0,
            magic_resist: 0.0,
        }
    }

    pub const fn base_stats(champion_id: ChampionId, level: u8, is_mega_gnar: bool) -> Self {
        let stats = match is_mega_gnar {
            true => MEGA_GNAR_BASE_STATS[const_clamp(level, 1..=URF_MAX_LEVEL as u8)],
            false => get_base_stats(champion_id, level),
        };
        Self {
            health: stats.health,
            armor: stats.armor,
            magic_resist: stats.magic_resist,
        }
    }
}

impl BasicStats<f32> {
    pub const fn default() -> Self {
        Self {
            health: 0.0,
            armor: 0.0,
            magic_resist: 0.0,
            attack_damage: 0.0,
            mana: 0.0,
        }
    }

    pub const fn base_stats(champion_id: ChampionId, level: u8, is_mega_gnar: bool) -> Self {
        match is_mega_gnar {
            true => MEGA_GNAR_BASE_STATS[const_clamp(level, 1..=URF_MAX_LEVEL as u8)],
            false => get_base_stats(champion_id, level),
        }
    }
}

pub const fn get_simulated_stats(stats: &Stats<f32>, dragons: Dragons) -> [Stats<f32>; L_SIML] {
    let mut result = MaybeUninit::<[Stats<f32>; L_SIML]>::uninit();
    let result_ptr = result.as_mut_ptr();

    let mut i = 0;
    while i < SIMULATED_ITEMS_ENUM.len() {
        let item_offset = SIMULATED_ITEMS_ENUM[i];
        let item_cache = INTERNAL_ITEMS[item_offset as usize];
        let mut new_stat = *stats;

        new_stat.armor_penetration_flat += item_cache.stats.armor_penetration_flat;
        new_stat.magic_penetration_flat += item_cache.stats.magic_penetration_flat;
        new_stat.ability_power += item_cache.stats.ability_power;
        new_stat.attack_damage += item_cache.stats.attack_damage;
        new_stat.magic_resist += item_cache.stats.magic_resist;
        new_stat.attack_speed += item_cache.stats.attack_speed;
        new_stat.crit_chance += item_cache.stats.crit_chance;
        new_stat.crit_damage += item_cache.stats.crit_damage;
        new_stat.health += item_cache.stats.health;
        new_stat.armor += item_cache.stats.armor;
        new_stat.mana += item_cache.stats.mana;
        new_stat.armor_penetration_percent = RiotFormulas::percent_value(&[
            new_stat.armor_penetration_percent,
            stats.armor_penetration_percent,
        ]);
        new_stat.magic_penetration_percent = RiotFormulas::percent_value(&[
            new_stat.magic_penetration_percent,
            stats.magic_penetration_percent,
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

pub fn get_runes_data(runes: &BitArray, attack_type: AttackType) -> DamageKind<L_RUNE, RuneId> {
    let mut metadata = SmallVec::with_capacity(runes.count() as usize);
    let mut closures = SmallVec::with_capacity(runes.count() as usize);
    for rune_number in runes.into_iter() {
        let rune = unsafe { INTERNAL_RUNES.get_unchecked(rune_number as usize) };
        closures.push(match attack_type {
            AttackType::Ranged => rune.range_closure,
            AttackType::Melee => rune.melee_closure,
        });
        metadata.push(rune.metadata);
    }
    DamageKind { metadata, closures }
}

const _: () = {
    let mut index = 0;
    while index < NUMBER_OF_ITEMS {
        let data = INTERNAL_ITEMS[index];
        assert!(data.melee_closure.len() <= 2);
        assert!(data.range_closure.len() <= 2);
        index += 1;
    }
};

pub fn get_items_data(
    items: &BitArray,
    attack_type: AttackType,
) -> (
    DamageKind<L_ITEM, ItemId>,
    SmallVec<[(usize, usize); L_ITEM]>,
) {
    let mut metadata = SmallVec::with_capacity(items.count() as usize);
    let mut closures = SmallVec::with_capacity(items.count() as usize);
    let mut multi_closure_indices = SmallVec::with_capacity(items.count() as usize);
    for (index, item_number) in items.into_iter().enumerate() {
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(item_number as usize) };
        let slice = match attack_type {
            AttackType::Ranged => item.range_closure,
            AttackType::Melee => item.melee_closure,
        };

        if slice.len() > 1 {
            multi_closure_indices.push((index, index + 1));
            metadata.push(item.metadata);
        }

        closures.extend_from_slice(slice);
        metadata.push(item.metadata);
    }
    (DamageKind { metadata, closures }, multi_closure_indices)
}

pub const fn runes_slice_to_bit_array(input: &[RuneId]) -> BitArray {
    let mut out = BitArray::EMPTY;
    let mut i = 0;
    while i < input.len() {
        let rune = input[i] as usize;
        if DAMAGING_RUNES.contains(rune) {
            let _ = out.insert(rune);
        }
        i += 1;
    }
    out
}

pub const fn items_slice_to_bit_array(input: &[ItemId]) -> BitArray {
    let mut out = BitArray::EMPTY;
    let mut i = 0;
    while i < input.len() {
        let item = input[i] as usize;
        if DAMAGING_ITEMS.contains(item) {
            let _ = out.insert(item);
        }
        i += 1;
    }
    out
}

pub const fn get_enemy_current_stats(
    stats: &mut SimpleStats<f32>,
    items: &BitArray,
    earth_dragons: u16,
) -> f32 {
    let mut bonus_mana = 0.0;

    let mut i = 0;
    let mut inner = items.into_inner();
    while i < items.count() as usize {
        if let Some(item_id) = bit_array_pop(&mut inner) {
            let item = INTERNAL_ITEMS[item_id];
            stats.magic_resist += item.stats.magic_resist;
            stats.health += item.stats.health;
            stats.armor += item.stats.armor;
            bonus_mana += item.stats.mana;
        }
        i += 1;
    }
    let dragon_mod = get_earth_multiplier(earth_dragons);
    stats.magic_resist *= dragon_mod;
    stats.armor *= dragon_mod;
    bonus_mana
}

pub const fn get_enemy_state(
    state: EnemyState,
    shred: ResistShred,
    accept_negatives: bool,
) -> EnemyFullState {
    let mut e_current_stats = state.base_stats;
    let e_items = &state.items;
    let stacks = state.stacks as f32;

    let bonus_mana = get_enemy_current_stats(&mut e_current_stats, &e_items, state.earth_dragons);

    let mut e_modifiers = DamageModifiers::default();

    let mut i = 0;
    while i < state.item_exceptions.len() {
        let item_exception = state.item_exceptions[i];
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
        i += 1;
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
            e_current_stats.armor *= ornn_resist_multiplier;
            e_current_stats.magic_resist *= ornn_resist_multiplier;
            e_current_stats.health *= ornn_resist_multiplier;
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
            &e_items,
            [
                ItemId::RanduinsOmen,
                ItemId::FrozenHeart,
                ItemId::WardensMail,
            ],
        ),
        // #![manual_impl]
        randuin: has_item(&e_items, [ItemId::RanduinsOmen]),
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

pub trait AbilityExt {
    fn apply_modifiers(&self, _: &mut f32, _: &AbilityModifiers) {}
}

impl AbilityExt for ItemId {}
impl AbilityExt for RuneId {}
impl AbilityExt for AbilityLike {
    fn apply_modifiers(&self, modifier: &mut f32, ability_modifiers: &AbilityModifiers) {
        let mut modify = |ability_name: AbilityName, value: f32| {
            // Any ability that is not Monster or Minion damage should have the modifier applied
            if ability_name <= AbilityName::Mega {
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

pub fn eval_damage<const N: usize, T: AbilityExt + 'static>(
    ctx: &EvalContext,
    onhit: &mut RangeDamage,
    metadata: &[TypeMetadata<T>],
    closures: &[ConstClosure],
    modifiers: Modifiers,
) -> SmallVec<[i32; N]> {
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

        let damage = (modifier * closure(ctx)) as i32;

        match attributes {
            Attrs::OnhitMin => onhit.minimum_damage += damage,
            Attrs::OnhitMax => onhit.maximum_damage += damage,
            Attrs::Onhit => {
                onhit.minimum_damage += damage;
                onhit.maximum_damage += damage;
            }
            _ => {}
        };

        result.push(damage);
    }
    result
}

pub const fn eval_attacks(ctx: &EvalContext, mut onhit_damage: RangeDamage) -> Attacks {
    let basic_attack = ctx.ad as i32;
    let critical_strike = (ctx.ad * ctx.crit_damage / 100.0) as i32;

    onhit_damage.minimum_damage += basic_attack;
    onhit_damage.maximum_damage += critical_strike;

    Attacks {
        basic_attack,
        critical_strike,
        onhit_damage,
    }
}

pub fn get_damages(eval_ctx: &EvalContext, data: &DamageEvalData, modifiers: Modifiers) -> Damages {
    let mut onhit = RangeDamage::default();

    let abilities = eval_damage(
        &eval_ctx,
        &mut onhit,
        &data.abilities.metadata,
        &data.abilities.closures,
        modifiers,
    );
    let items = eval_damage(
        &eval_ctx,
        &mut onhit,
        &data.items.metadata,
        &data.items.closures,
        modifiers,
    );
    let runes = eval_damage(
        &eval_ctx,
        &mut onhit,
        &data.runes.metadata,
        &data.runes.closures,
        modifiers,
    );
    let attacks = eval_attacks(&eval_ctx, onhit);

    Damages {
        abilities,
        items,
        runes,
        attacks,
    }
}
