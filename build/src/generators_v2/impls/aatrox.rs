use super::*;

impl GeneratorV2 for Aatrox {
    fn generate(&mut self) -> MayFail {
        // The bonus magic damage, then its cap against monsters
        self.take_nth("Innate", 0, P(Void))?
            .take("First Cast Damage", Q(_1Min))?
            .take("First Sweetspot Damage", Q(_1Max))?
            .take("Second Cast Damage", Q(_2Min))?
            .take("Second Sweetspot Damage", Q(_2Max))?
            .take("Third Cast Damage", Q(_3Min))?
            .take("Third Sweetspot Damage", Q(_3Max))?
            // The three casts together
            .take("Maximum Non-Minion Non-Sweetspot Damage", Q(Min))?
            .take("Maximum Non-Minion Sweetspot Damage", Q(Max))?
            .take("Physical Damage", W(Min))?
            .take("Total Damage", W(Max))?
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
