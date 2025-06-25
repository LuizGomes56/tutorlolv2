#[allow(non_snake_case)]
pub struct EvalContext {
    pub CHOGATH_STACKS: f64,
    pub VEIGAR_STACKS: f64,
    pub NASUS_STACKS: f64,
    pub SMOLDER_STACKS: f64,
    pub AURELION_SOL_STACKS: f64,
    pub THRESH_STACKS: f64,
    pub KINDRED_STACKS: f64,
    pub BELVETH_STACKS: f64,
    pub ADAPTATIVE_DAMAGE: f64,
    pub LEVEL: f64,
    pub PHYSICAL_MULTIPLIER: f64,
    pub MAGIC_MULTIPLIER: f64,
    pub STEELCAPS_EFFECT: f64,
    pub RANDUIN_EFFECT: f64,
    pub ROCKSOLID_EFFECT: f64,
    pub ENEMY_BONUS_HEALTH: f64,
    pub ENEMY_ARMOR: f64,
    pub ENEMY_MAX_HEALTH: f64,
    pub ENEMY_HEALTH: f64,
    pub ENEMY_CURRENT_HEALTH: f64,
    pub ENEMY_MISSING_HEALTH: f64,
    pub ENEMY_MAGIC_RESIST: f64,
    pub BASE_HEALTH: f64,
    pub BASE_AD: f64,
    pub BASE_ARMOR: f64,
    pub BASE_MAGIC_RESIST: f64,
    pub BASE_MANA: f64,
    pub BONUS_AD: f64,
    pub BONUS_ARMOR: f64,
    pub BONUS_MAGIC_RESIST: f64,
    pub BONUS_HEALTH: f64,
    pub BONUS_MANA: f64,
    pub BONUS_MOVE_SPEED: f64,
    pub ARMOR_PENETRATION_FLAT: f64,
    pub ARMOR_PENETRATION_PERCENT: f64,
    pub MAGIC_PENETRATION_FLAT: f64,
    pub MAGIC_PENETRATION_PERCENT: f64,
    pub MAX_MANA: f64,
    pub CURRENT_MANA: f64,
    pub MAX_HEALTH: f64,
    pub CURRENT_HEALTH: f64,
    pub ARMOR: f64,
    pub MAGIC_RESIST: f64,
    pub CRIT_CHANCE: f64,
    pub CRIT_DAMAGE: f64,
    pub ATTACK_SPEED: f64,
    pub MISSING_HEALTH: f64,
    pub AP: f64,
    pub AD: f64,
}

pub struct CachedChampion {
    pub name: &'static str,
    pub adaptative_type: &'static str,
    pub attack_type: &'static str,
    pub positions: &'static [&'static str],
    pub stats: CachedChampionStats,
    pub abilities: &'static [(&'static str, CachedChampionAbility)],
}

pub struct CachedChampionAbility {
    pub name: &'static str,
    pub damage_type: &'static str,
    pub damages_in_area: bool,
    pub minimum_damage: fn(usize, &EvalContext) -> f64,
    pub maximum_damage: fn(usize, &EvalContext) -> f64,
}

pub struct CachedChampionStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resistance: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: CachedChampionStatsMap,
    pub critical_strike_damage: CachedChampionStatsMap,
    pub critical_strike_damage_modifier: CachedChampionStatsMap,
    pub attack_speed_ratio: CachedChampionStatsMap,
    pub attack_range: CachedChampionStatsMap,
    pub aram_damage_taken: CachedChampionStatsMap,
    pub aram_damage_dealt: CachedChampionStatsMap,
    pub urf_damage_taken: CachedChampionStatsMap,
    pub urf_damage_dealt: CachedChampionStatsMap,
}

pub struct CachedItemDamages {
    pub minimum_damage: fn(usize, &EvalContext) -> f64,
    pub maximum_damage: fn(usize, &EvalContext) -> f64,
}

pub struct CachedItem {
    pub name: &'static str,
    pub gold: usize,
    pub tier: usize,
    pub prettified_stats: &'static [(&'static str, f64)],
    pub damage_type: Option<&'static str>,
    pub stats: CachedItemStats,
    pub builds_from: &'static [usize],
    pub levelings: Option<&'static [usize]>,
    pub ranged: CachedItemDamages,
    pub melee: CachedItemDamages,
    pub damages_onhit: bool,
}

pub struct CachedMetaItem {
    pub jungle: &'static [usize],
    pub top: &'static [usize],
    pub mid: &'static [usize],
    pub adc: &'static [usize],
    pub support: &'static [usize],
}

pub struct CachedRune {
    pub name: &'static str,
    pub damage_type: &'static str,
    pub ranged: fn(usize, &EvalContext) -> f64,
    pub melee: fn(usize, &EvalContext) -> f64,
}

pub struct CachedItemStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_percent: f64,
    pub armor_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub critical_strike_chance: f64,
    pub critical_strike_damage: f64,
    pub health: f64,
    pub lifesteal: f64,
    pub magic_resistance: f64,
    pub mana: f64,
    pub movespeed: f64,
    pub omnivamp: f64,
}

pub struct GlobalCache {
    pub champions: &'static phf::Map<&'static str, &'static CachedChampion>,
    pub items: &'static phf::Map<usize, &'static CachedItem>,
    pub runes: &'static phf::Map<usize, &'static CachedRune>,
    pub meta_items: &'static phf::Map<&'static str, &'static CachedMetaItem>,
    pub champion_names: &'static phf::Map<&'static str, &'static str>,
    pub simulated_items: &'static [usize],
}
