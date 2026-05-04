use super::*;

impl Generator for Aatrox {
    fn generate(&mut self) -> MayFail {
        // self.passive(Void, (0, 0), Some("".times(EnemyBonusHealth)), None)
        //     .ability(
        //         Key::Q,
        //         [
        //             (2, 0, _1Min),
        //             (2, 1, _1Max),
        //             (3, 0, _2Min),
        //             (3, 1, _2Max),
        //             (5, 0, _3Min),
        //             (5, 1, _3Max),
        //         ],
        //     )
        //     .ability(Key::W, [(0, 1, Min), (1, 0, Max)]);

        // let q = &self[Q(_1Min)];

        // let q_min = Ability {
        //     damage: self.sum([Q(_1Min), Q(_2Min), Q(_3Min)])?,
        //     ..q.clone()
        // };

        // let q_max = Ability {
        //     damage: self.sum([Q(_1Max), Q(_2Max), Q(_3Max)])?,
        //     ..q.clone()
        // };

        // self.insert(Q(Min), q_min)
        //     .insert(Q(Max), q_max)
        self.attr(
            Area,
            [
                Q(_1Min),
                Q(_1Max),
                Q(_2Min),
                Q(_2Max),
                Q(_3Min),
                Q(_3Max),
                Q(Min),
                Q(Max),
            ],
        )?
        .combo([
            Ability(Q(_1Min)),
            Ability(P(Void)),
            Attack,
            Ability(W(Min)),
            Ability(Q(_2Min)),
            Attack,
            Ability(W(Min)),
            Ability(Q(_3Min)),
            Attack,
        ])?
        .combo([
            Ability(Q(_1Max)),
            Ability(P(Void)),
            Attack,
            Ability(W(Min)),
            Ability(Q(_2Max)),
            Attack,
            Ability(W(Min)),
            Ability(Q(_3Max)),
            Attack,
        ])?
        .progress(Preserve)
        .end()
    }
}
