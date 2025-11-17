use super::*;

// #![stable]
// #![allow_missing_offsets]

// pub fn test<const N: usize>(a: fn([i32; N]) -> Vec<String>, b: [i32; N]) {
//     a(b);
// }

// pub fn t() {
//     test(
//         |[a, b, c]| vec![a.to_string(), b.to_string(), c.to_string()],
//         [1, 2, 3],
//     );
// }

impl Generator<Champion> for Aatrox {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P::Void, (0, 0), EnemyBonusHealth];
        self.ability(
            Q,
            [
                (2, 0, _1Min),
                (2, 1, _1Max),
                (3, 0, _2Min),
                (3, 1, _2Max),
                (5, 0, _3Min),
                (5, 1, _3Max),
            ],
        );
        self.ability(W, [(0, 1, Min), (1, 0, Max)]);

        attr![
            Area => [
                Q::_1Min,
                Q::_1Max,
                Q::_2Min,
                Q::_2Max,
                Q::_3Min,
                Q::_3Max,
            ]
        ];

        let default_ability = self.get(Q::_1)?.clone();

        let q_min = Ability {
            damage: merge_damage!(
                |q1, q2, q3| format!("({q1}) + ({q2}) + ({q3})"),
                Q::_1Min,
                Q::_2Min,
                Q::_3Min,
            ),
            ..default_ability.clone()
        };

        let q_max = Ability {
            damage: merge_damage!(
                |q1, q2, q3| format!("({q1}) + ({q2}) + ({q3})"),
                Q::_1Max,
                Q::_2Max,
                Q::_3Max,
            ),
            ..default_ability
        };

        self.insert(Q::Min, q_min);
        self.insert(Q::Max, q_max);
    }
}
