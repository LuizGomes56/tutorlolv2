use super::*;

// #![stable]
// #![allow_missing_offsets]

impl Generator<Champion> for Aatrox {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P::Void, (0, 0), EnemyBonusHealth];
        ability![
            Q,
            (2, 0, _1),
            (2, 1, _1Max),
            (3, 0, _2),
            (3, 1, _2Max),
            (5, 0, _3),
            (5, 1, _3Max)
        ];
        ability![W, (0, 1, Void), (1, 0, Max)];

        let default_ability = get![Q::_1].clone();

        insert!(
            Q::Void,
            Ability {
                damage: merge_damage!(
                    |q1, q2, q3| format!("({q1}) + ({q2}) + ({q3})"),
                    Q::_1,
                    Q::_2,
                    Q::_3,
                ),
                ..default_ability.clone()
            }
        );

        insert!(
            Q::Max,
            Ability {
                damage: merge_damage!(
                    |q1, q2, q3| format!("({q1}) + ({q2}) + ({q3})"),
                    Q::_1Max,
                    Q::_2Max,
                    Q::_3Max,
                ),
                ..default_ability
            }
        );

        merge![
            Q::_1 - Q::_1Max,
            Q::_2 - Q::_2Max,
            Q::_3 - Q::_3Max,
            Q::Void - Q::Max,
            W::Void - W::Max
        ];
    }
}
