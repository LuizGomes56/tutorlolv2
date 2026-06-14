pub const TOWER_DAMAGE: &str = r#"intrinsic TOWER_DAMAGE {
    damage_type: RiotFormulas::adaptive_type(
        bonus_stats.attack_damage,
        current_stats.ability_power,
    ),
    definition: const fn get_tower_damages(
        AdaptiveType,
        ResistShred,
        ...f32
    ) -> [i32; L_TWRD]
}"#;

pub const IGNITE_FN: &str = r#"fn ignite(level: i32) -> i32 {
    70 + 20 * level + 5
      * if level > 4 { level - 4 }
        else { 0 }
}"#;

pub const TOWER_DAMAGE_FN: &str = r#"fn tower_damage(...) -> i32 {
    let base = base_attack_damage
        + bonus_attack_damage
        + ability_power * 0.6;
    let bonus_resist = match plates == 0 {
        true => 0.0,
        false => -25 + 50 * (plates - 1),
    };
    let raw_resist = 40 + bonus_resist;
    let resist = raw_resist
        * (1 - pen_percent / 100)
        - pen_flat;
    let mult = 100 / (100 + resist);
    base * mult
}"#;

pub const ONHIT_EFFECT: &str = r#"intrinsic ONHIT_EFFECT {
    damage_type: Mixed,
    definition: fn onhit(...) -> Attacks
};"#;

pub const ONHIT_EFFECT_FN: &str = r#"fn onhit() -> Attacks {
    intrinsic
}"#;

pub const CRITICAL_STRIKE: &str = r#"intrinsic CRITICAL_STRIKE {
    attributes: OnhitMax,
    damage_type: Physical,
    damage: attack_damage * crit_damage / 100
};"#;

pub const CRITICAL_STRIKE_FN: &str = r#"fn critical_strike() -> f32 {
    attack_damage * crit_damage
      / 100 /* * physical_multiplier */
}"#;

pub const BASIC_ATTACK: &str = r#"intrinsic BASIC_ATTACK {
    attributes: OnhitMin,
    damage_type: Physical,
    damage: attack_damage /* * physical_multiplier */,
};"#;

pub const BASIC_ATTACK_FN: &str = r#"fn basic_attack() -> f32 {
    attack_damage /* * physical_multiplier */
}"#;
