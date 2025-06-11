use super::{
    Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage, extract_passive_damage,
};

// #![stable]
// #![since] "06/01/2025"
// #![patch] "25.9"
// * Q_MAX was intentionally placed at position "minimum_damage"
// * Passive postfix "ENEMY_MAX_HEALTH" need manual fix if Riot changes it
// * Minion and Monster bonus damages are omitted in version 0.1.0

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    passive!(
        "P",
        (0, 0),
        Target::MINIMUM,
        (None, Some("ENEMY_MAX_HEALTH"))
    );
    ability!(
        q,
        (2, 0, "Q1", Target::MINIMUM),
        (2, 1, "Q1_MAX", Target::MAXIMUM),
        (3, 0, "Q2", Target::MINIMUM),
        (3, 1, "Q2_MAX", Target::MAXIMUM),
        (4, 0, "Q3", Target::MINIMUM),
        (4, 1, "Q3_MAX", Target::MAXIMUM),
    );
    ability!(
        w,
        (0, 0, "W", Target::MINIMUM),
        (0, 1, "W_MINION", Target::MINIMUM),
        (2, 0, "W_MAX", Target::MAXIMUM),
    );

    let mut insert_max = |keyname: &str| {
        let max_damage = abilities
            .get(&format!("{}_MAX", keyname))
            .unwrap()
            .maximum_damage
            .clone();
        let map_mut_ref = abilities.get_mut(keyname).unwrap();
        map_mut_ref.maximum_damage = max_damage;
        abilities.remove(&format!("{}_MAX", keyname));
    };

    insert_max("Q1");
    insert_max("Q2");
    insert_max("Q3");

    let default_ability = abilities.get("Q1").unwrap().clone();

    let [q1_max_damage, q2_max_damage, q3_max_damage] =
        ["Q1", "Q2", "Q3"].map(|key| abilities.get(key).unwrap().maximum_damage.clone());

    let mut q_max_damage = Vec::<String>::new();

    for i in 0..q1_max_damage.len() {
        q_max_damage.push(format!(
            "({}) + ({}) + ({})",
            q1_max_damage[i], q2_max_damage[i], q3_max_damage[i]
        ));
    }

    abilities.insert(
        String::from("Q_MAX"),
        Ability {
            minimum_damage: q_max_damage,
            ..default_ability
        },
    );

    let w_max = abilities.get("W_MAX").unwrap().maximum_damage.clone();
    let w_mut_ref = abilities.get_mut("W").unwrap();
    w_mut_ref.maximum_damage = w_max;
    abilities.remove("W_MAX");
}
