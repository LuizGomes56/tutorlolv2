use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_aatrox(data: CdnChampion) -> Champion {
    passive!(Void, (0, 0), (None, Some("ENEMY_MAX_HEALTH")));
    ability!(w, (0, 0, Void), (0, 1, Minion), (2, 0, Max));
    ability!(
        q,
        (2, 0, _1),
        (2, 1, _1Max),
        (3, 0, _2),
        (3, 1, _2Max),
        (5, 0, _3),
        (5, 1, _3Max)
    );
    ability!(w, (0, 1, Void), (1, 0, Max));

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
