use super::*;

impl GeneratorV2 for Hecarim {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            // Rampage bonus damage per stack, on every mode
            .edit(Q(Void), |q| f![q * Stacks * (1.03 + 0.03 * BonusAd / 100)])
            .take("Magic Damage Per Tick", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Maximum Physical Damage", E(Max))?
            .take("Minimum Physical Damage", E(Min))?
            .take("Magic damage", R(Void))?
            .combos([
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
