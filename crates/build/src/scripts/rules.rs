//! Generates the constants of the drakes, the stat shards, ARAM: Mayhem and turrets.

use crate::MayFail;
use wiki::rules::Rules;

pub fn rules_code(rules: &Rules) -> MayFail<String> {
    let Rules {
        drakes,
        shards,
        aram_mayhem,
        turret,
        move_speed,
    } = rules;

    let float = |value: f64| format!("{:?}f32", value as f32);

    Ok(format!(
        "pub mod rules {{
            /// Elemental drakes, ratios granted by each stack
            pub mod drakes {{
                /// Attack damage and ability power of Infernal Might
                pub const INFERNAL_MIGHT: f32 = {};
                /// Armor and magic resistance of Mountainous Vigor
                pub const MOUNTAINOUS_VIGOR: f32 = {};
            }}

            /// Stat shards of the rune page
            pub mod shards {{
                pub const ADAPTIVE_FORCE: f32 = {};
                /// Percent of bonus attack speed
                pub const ATTACK_SPEED: f32 = {};
                pub const HEALTH: f32 = {};
                /// Bonus health per level
                pub const HEALTH_SCALING_PER_LEVEL: f32 = {};
                /// Percent of bonus movement speed
                pub const MOVE_SPEED: f32 = {};
            }}

            /// Global stat changes of ARAM: Mayhem
            pub mod aram_mayhem {{
                /// Adaptive force for every 1% critical strike chance above 100%
                pub const EXCESS_CRIT_ADAPTIVE_FORCE: f32 = {};
                /// Base health from level 1 to level 18
                pub const MELEE_BASE_HEALTH: [f32; 2] = [{}, {}];
                pub const RANGED_BASE_HEALTH: [f32; 2] = [{}, {}];
                /// Bonus armor and magic resistance
                pub const MELEE_BONUS_RESISTS: f32 = {};
                /// Bonus armor and magic resistance from level 1 to level 18
                pub const RANGED_BONUS_RESISTS: [f32; 2] = [{}, {}];
            }}

            /// Outer turret of Summoner's Rift, with only the attacker nearby
            pub mod turret {{
                /// Armor and magic resistance
                pub const RESISTS: f32 = {};
                /// Ability power ratio of basic attacks against turrets
                pub const AP_RATIO: f32 = {};
                /// Bonus armor and magic resistance of each Bulwark stack
                pub const BULWARK_RESISTS_PER_STACK: f32 = {};
                pub const BULWARK_MAX_STACKS: usize = {};
            }}

            /// Soft caps of movement speed
            pub mod move_speed {{
                /// Raw speed above it is multiplied by `FIRST_RATIO`
                pub const FIRST_CAP: f32 = {};
                pub const FIRST_RATIO: f32 = {};
                /// Raw speed above it is multiplied by `SECOND_RATIO`
                pub const SECOND_CAP: f32 = {};
                pub const SECOND_RATIO: f32 = {};
                /// Raw speed below it becomes `LOWER_FLOOR + raw * LOWER_RATIO`
                pub const LOWER_CAP: f32 = {};
                pub const LOWER_FLOOR: f32 = {};
                pub const LOWER_RATIO: f32 = {};
            }}
        }}",
        float(drakes.infernal),
        float(drakes.mountain),
        float(shards.adaptive_force),
        float(shards.attack_speed),
        float(shards.health),
        float(shards.health_scaling_per_level),
        float(shards.move_speed),
        float(aram_mayhem.excess_crit_adaptive_force),
        float(aram_mayhem.melee_base_health.min),
        float(aram_mayhem.melee_base_health.max),
        float(aram_mayhem.ranged_base_health.min),
        float(aram_mayhem.ranged_base_health.max),
        float(aram_mayhem.melee_bonus_resists),
        float(aram_mayhem.ranged_bonus_resists.min),
        float(aram_mayhem.ranged_bonus_resists.max),
        float(turret.resists),
        float(turret.ap_ratio),
        float(turret.bulwark_resists_per_stack),
        turret.bulwark_max_stacks,
        float(move_speed.first_cap),
        float(move_speed.first_ratio),
        float(move_speed.second_cap),
        float(move_speed.second_ratio),
        float(move_speed.lower_cap),
        float(move_speed.lower_floor),
        float(move_speed.lower_ratio),
    ))
}
