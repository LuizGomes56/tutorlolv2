use super::{
    Ability, CdnChampion, Champion, FxHashMap, Target, extract_ability_damage,
    extract_passive_damage,
};

// #![stable] "06/11/2025" | "25.11"
// #![unsupported] MINION | MONSTER
/// * Q_MAX was intentionally placed at position "minimum_damage"
/// * Passive postfix "ENEMY_MAX_HEALTH" need manual fix if Riot changes it
/// * Minion and Monster bonus damages are omitted in version 0.1.0

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

    merge_ability!("Q1");
    merge_ability!("Q2");
    merge_ability!("Q3");
    merge_ability!("W");

    let default_ability = get!("Q1").clone();

    abilities.insert(
        String::from("Q_MAX"),
        Ability {
            minimum_damage: merge_damage!(
                || format!("({}) + ({}) + ({})", q1, q2, q3),
                (q1, maximum_damage),
                (q2, maximum_damage),
                (q3, maximum_damage)
            ),
            maximum_damage: Vec::new(),
            ..default_ability
        },
    );
}
