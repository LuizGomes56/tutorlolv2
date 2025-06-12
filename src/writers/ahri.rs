use super::{Ability, CdnChampion, Champion, FxHashMap, Target, extract_ability_damage};

// #![stable] "06/11/2025" | "25.11"
// #![unsupported] MINION | MONSTER

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM),);
    ability!(
        w,
        (1, 0, "W", Target::MINIMUM),
        (1, 1, "W1", Target::MINIMUM),
        (1, 2, "W_MAX", Target::MAXIMUM),
        (3, 0, "W_MINION", Target::MINIMUM),
        (3, 1, "W_MINION_MAX", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));

    merge_ability!("W");
    merge_ability!("W_MINION");

    let q_value = abilities.get("Q").unwrap().clone();
    let q_mut_ref = abilities.get_mut("Q").unwrap();
    q_mut_ref.maximum_damage = q_value
        .minimum_damage
        .iter()
        .map(|value| format!("({}) * MAGIC_MULTIPLIER + ({})", value, value))
        .collect();
    q_mut_ref.damage_type = String::from("MIXED");

    let r_value = abilities.get("R").unwrap().clone();
    abilities.get_mut("R").unwrap().maximum_damage = r_value
        .minimum_damage
        .iter()
        .map(|value| format!("3 * ({})", value))
        .collect();
}
