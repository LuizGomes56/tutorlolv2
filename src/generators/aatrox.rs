use super::*;

// #![stable] "08/07/2025" | "25.15"
/// * Q_MAX was intentionally placed at position "minimum_damage"
/// * Passive postfix "ENEMY_MAX_HEALTH" need manual fix if Riot changes it
/// * Minion and Monster bonus damages are omitted in version 0.1.0

#[generator_macros::generator]
pub fn gen_aatrox(data: CdnChampion) -> Champion {
    passive!(Void, (0, 0), Min, (None, Some("ENEMY_MAX_HEALTH")));
    ability!(w, (0, 0, Void, Min), (0, 1, Minion, Min), (2, 0, Max, Max));
    ability!(
        q,
        (2, 0, _1, Min),
        (2, 1, _1Max, Max),
        (3, 0, _2, Min),
        (3, 1, _2Max, Max),
        (5, 0, _3, Min),
        (5, 1, _3Max, Max)
    );
    ability!(w, (0, 1, Void, Min), (1, 0, Max, Max));

    merge_ability!((Q, _1), (Q, _1Max));
    merge_ability!((Q, _2), (Q, _2Max));
    merge_ability!((Q, _3), (Q, _3Max));
    merge_ability!((W, Void), (W, Max));

    let default_ability = get!(Q, _1).clone();

    insert!(
        (Q, Max),
        Ability {
            minimum_damage: merge_damage!(
                5,
                |(q1, q2, q3)| format!("({}) + ({}) + ({})", q1, q2, q3),
                ((Q, _1), maximum_damage),
                ((Q, _2), maximum_damage),
                ((Q, _3), maximum_damage)
            ),
            maximum_damage: Vec::new(),
            ..default_ability
        }
    );
}
