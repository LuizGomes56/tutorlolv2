use super::*;

// #![stable]

impl Generator for Aatrox {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (2, 0, _1),
            (2, 1, _2),
            (3, 0, _3),
            (3, 1, _4),
            (4, 0, _5),
            (4, 1, _6),
            (5, 0, _7),
            (5, 1, _8)
        ];
        ability![W, (0, 0, _1), (0, 1, _2), (2, 0, _3)];
        ability![R, (2, 0, _1)];

        let default_ability = get![Q::_1].clone();

        let q_max_damage = merge_damage!(
            |q1, q2, q3| format!("({}) + ({}) + ({})", q1, q2, q3),
            Q::_1,
            Q::_2,
            Q::_3,
        );

        insert!(
            Q::Max,
            Ability {
                damage: q_max_damage,
                ..default_ability
            }
        );

        merge![
            Q::_1 <= Q::_2,
            Q::_3 <= Q::_4,
            Q::_5 <= Q::_6,
            W::_1 <= W::_2,
        ];
    }
}
