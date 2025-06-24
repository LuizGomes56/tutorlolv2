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
    pub minimum_damage: &'static [&'static str],
    pub maximum_damage: &'static [&'static str],
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
    pub minimum_damage: Option<&'static str>,
    pub maximum_damage: Option<&'static str>,
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
    pub ranged: Option<CachedItemDamages>,
    pub melee: Option<CachedItemDamages>,
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
    pub ranged: &'static str,
    pub melee: &'static str,
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
