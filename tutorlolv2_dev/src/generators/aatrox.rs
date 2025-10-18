use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_aatrox(data: CdnChampion) -> Champion {
    passive!(Void, (0, 0), (None, Some("ENEMY_MAX_HEALTH")));
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

    let default_ability = get!(Q::_1).clone();

    insert!(
        Q::Max,
        Ability {
            damage: merge_damage!(
                5,
                |(q1, q2, q3)| format!("({}) + ({}) + ({})", q1, q2, q3),
                Q::_1,
                Q::_2,
                Q::_3,
            ),
            ..default_ability
        }
    );
}
