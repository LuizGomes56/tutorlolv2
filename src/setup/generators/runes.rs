// Generate fields "melee" and "ranged" for each rune in "runes.json".

fn _rune_8005() {}
fn _rune_8112() {}
fn _rune_8124() {}
fn _rune_8126() {}
fn _rune_8128() {}
fn _rune_8143() {}

// #![manual_impl]
// #![unstable] "05/24/2024" | "14.5"
fn __hc_tutorlolv1_js() {
    br#"{
    "8005": {
        "name": "Press The Attack",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(40 + ((120 / 17) * (LEVEL - 1))) * ADAPTATIVE_DAMAGE",
        "ranged": "(40 + ((120 / 17) * (LEVEL - 1))) * ADAPTATIVE_DAMAGE"
    },
    "8112": {
        "name": "Electrocute",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(30 + ((190 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(30 + ((190 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8124": {
        "name": "Predator",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(20 + ((160 / 17) * (LEVEL - 1)) + ((0.25 * BONUS_AD + 0.15 * AP))) * ADAPTATIVE_DAMAGE",
        "ranged": "(20 + ((160 / 17) * (LEVEL - 1)) + ((0.25 * BONUS_AD + 0.15 * AP))) * ADAPTATIVE_DAMAGE"
    },
    "8126": {
        "name": "Cheap Shot",
        "damage_type": "TRUE_DAMAGE",
        "melee": "10 + ((35 / 17) * (LEVEL - 1))",
        "ranged": "10 + ((35 / 17) * (LEVEL - 1))"
    },
    "8128": {
        "name": "Dark Harvest",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(20 + ((60 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(20 + ((60 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8143": {
        "name": "Sudden Impact",
        "damage_type": "TRUE_DAMAGE",
        "melee": "20 + (60 / 17 * (LEVEL - 1))",
        "ranged": "20 + (60 / 17 * (LEVEL - 1))"
    },
    "8214": {
        "name": "Aery",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(10 + ((40 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(10 + ((40 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8229": {
        "name": "Comet",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(30 + ((100 / 17) * (LEVEL - 1)) + ((0.1 * BONUS_AD) + (0.05 * AP))) * ADAPTATIVE_DAMAGE",
        "ranged": "(30 + ((100 / 17) * (LEVEL - 1)) + ((0.1 * BONUS_AD) + (0.05 * AP))) * ADAPTATIVE_DAMAGE"
    },
    "8237": {
        "name": "Scorch",
        "damage_type": "MAGIC_DAMAGE",
        "melee": "(20 + ((20 / 17) * (LEVEL - 1))) * MAGIC_MULTIPLIER",
        "ranged": "(20 + ((20 / 17) * (LEVEL - 1))) * MAGIC_MULTIPLIER"
    },
    "8437": {
        "name": "Grasp",
        "damage_type": "MAGIC_DAMAGE",
        "melee": "(0.035 * MAX_HEALTH) * MAGIC_MULTIPLIER",
        "ranged": "(0.021 * MAX_HEALTH) * MAGIC_MULTIPLIER"
    },
    "8439": {
        "name": "Aftershock",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(25 + ((95 / 17) * (LEVEL - 1)) + (0.08 * BONUS_HEALTH)) * MAGIC_MULTIPLIER",
        "ranged": "(25 + ((95 / 17) * (LEVEL - 1)) + (0.08 * BONUS_HEALTH)) * MAGIC_MULTIPLIER"
    }
}"#;
}
