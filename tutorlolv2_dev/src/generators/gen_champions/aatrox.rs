use super::*;

// #![stable]
// #![allow_missing_offsets]

impl Generator<Champion> for Aatrox {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P::Void, (0, 0), EnemyBonusHealth];
        ability![
            Q,
            (2, 0, _1Min),
            (2, 1, _1Max),
            (3, 0, _2Min),
            (3, 1, _2Max),
            (5, 0, _3Min),
            (5, 1, _3Max)
        ];
        ability![W, (0, 1, Min), (1, 0, Max)];

        attr![
            Area,
            Q::_1Min,
            Q::_1Max,
            Q::_2Min,
            Q::_2Max,
            Q::_3Min,
            Q::_3Max,
        ];

        let default_ability = get![Q::_1Min].clone();

        insert!(
            Q::Min,
            Ability {
                damage: merge_damage!(
                    |q1, q2, q3| format!("({q1}) + ({q2}) + ({q3})"),
                    Q::_1Min,
                    Q::_2Min,
                    Q::_3Min,
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
            Q::_1Min - Q::_1Max,
            Q::_2Min - Q::_2Max,
            Q::_3Min - Q::_3Max,
            Q::Min - Q::Max,
            W::Min - W::Max
        ];
    }
}
