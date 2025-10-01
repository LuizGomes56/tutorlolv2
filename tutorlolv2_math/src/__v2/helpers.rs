use super::{formulas::*, model::*, riot::*};
use std::mem::MaybeUninit;
use tutorlolv2_gen::{CachedChampionStatsMap, INTERNAL_ITEMS, ItemId, SIMULATED_ITEMS};

/// By 06/07/2025 Earth dragons give +5% resists
// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;

#[inline(always)]
pub fn get_base_stats(stats: &CachedChampionStatsMap, level: u8) -> f32 {
    RiotFormulas::stat_growth(stats.flat, stats.per_level, level)
}

#[inline]
pub fn get_simulated_stats(
    stats: &RiotChampionStats,
    dragons: Dragons,
) -> [RiotChampionStats; L_SIML] {
    let mut result_slice = MaybeUninit::uninit();

    for item_offset in SIMULATED_ITEMS.into_iter() {
        unsafe {
            let item_id = std::mem::transmute::<u16, ItemId>(*item_offset as u16);
            let item_cache = INTERNAL_ITEMS.get_unchecked(item_id as usize);
            let mut new_stat = *stats;

            macro_rules! add_stat {
                ($field:ident) => {
                    new_stat.$field += item_cache.stats.$field;
                };
                (@$field:ident) => {
                    new_stat.$field = RiotFormulas::percent_value([new_stat.$field, stats.$field]);
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

            new_stat.ability_power *= dragons.fire as f32 * FIRE_DRAGON_MULTIPLIER;
            new_stat.attack_damage *= dragons.fire as f32 * FIRE_DRAGON_MULTIPLIER;
            new_stat.armor *= dragons.earth as f32 * EARTH_DRAGON_MULTIPLIER;
            new_stat.magic_resist *= dragons.earth as f32 * EARTH_DRAGON_MULTIPLIER;
        }
    }

    unsafe {
        let result = result_slice.assume_init();
        result
    }
}
