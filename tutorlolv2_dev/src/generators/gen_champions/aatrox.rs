use super::*;

impl Generator for Aatrox {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate */
                (2, _2), /* Innate [1] */
                (3, _3), /* Innate [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1Min), /* First Cast Damage */
                (1, _1Max), /* First Sweetspot Damage */
                (4, _2Min), /* Second Cast Damage */
                (5, _2Max), /* Second Sweetspot Damage */
                (6, _3Min), /* Third Cast Damage */
                (7, _3Max), /* Third Sweetspot Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (1, Min), /* Physical Damage */
                (3, Max), /* Total Damage */
            ],
        );

        let qmin = self.sum([Q(_1Min), Q(_2Min), Q(_3Min)])?;
        let qmax = self.sum([Q(_1Max), Q(_2Max), Q(_3Max)])?;

        self.clone_to(Q(_1Min), Q(Min), qmin)?
            .clone_to(Q(_1Max), Q(Max), qmax)?
            .attr(
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
            .progress(Stable)
            .end()
    }
}
