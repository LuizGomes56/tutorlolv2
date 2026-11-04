use super::*;

impl Generator for Aatrox {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(
                Key::Q,
                [
                    ("First Cast Damage", _1Min),
                    ("First Sweetspot Damage", _1Max),
                    ("Second Cast Damage", _2Min),
                    ("Second Sweetspot Damage", _2Max),
                    ("Third Cast Damage", _3Min),
                    ("Third Sweetspot Damage", _3Max),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Physical Damage", Min),
                    ("Total Damage", Max),
                ],
            );

        let qmin = self.sum([Q(_1Min), Q(_2Min), Q(_3Min)])?;
        let qmax = self.sum([Q(_1Max), Q(_2Max), Q(_3Max)])?;

        self.clone_to(Q(_1Min), Q(Min), qmin)?
            .clone_to(Q(_1Max), Q(Max), qmax)?
            .combos([
                &[
                    Ability(Q(_1Min)),
                    Ability(P(Void)),
                    Attack,
                    Ability(W(Min)),
                    Ability(Q(_2Min)),
                    Attack,
                    Ability(W(Min)),
                    Ability(Q(_3Min)),
                    Attack,
                ],
                &[
                    Ability(Q(_1Max)),
                    Ability(P(Void)),
                    Attack,
                    Ability(W(Min)),
                    Ability(Q(_2Max)),
                    Attack,
                    Ability(W(Min)),
                    Ability(Q(_3Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
