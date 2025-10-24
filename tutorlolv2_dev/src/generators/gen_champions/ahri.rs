use super::*;

// #![stable]

impl Generator<Champion> for Ahri {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![
            W,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (3, 0, _4),
            (3, 1, _5)
        ];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];

        // let q_max = merge_damage!(
        //     |q| format!("({q}) * {} + ({q})", EvalIdent::MagicMultiplier),
        //     Q::Void,
        // );

        // let q_mut_ref = clone_to![Q::Void => Q::Max];
        // q_mut_ref.damage = q_max;
        // q_mut_ref.damage_type = DamageType::Mixed;

        // clone_to![R::Void => R::Max];
        // let r_max = merge_damage!(|r| format!("3 * ({r})"), R::Void);
        // get_mut![R::Max].damage = r_max;

        // merge![
        //     Q::Void <= Q::Max,
        //     W::Void <= W::Max,
        //     W::Minion <= W::MinionMax,
        //     R::Void <= R::Max
        // ];
    }
}
