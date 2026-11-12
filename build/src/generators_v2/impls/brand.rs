use super::*;

impl GeneratorV2 for Brand {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Ablaze", 0, P(Void))?
            .take_nth("Ablaze", 6, P(_1))?
            .set(
                P(_1),
                f![(0.06 + 0.06 / 17 * (Level - 1) + 0.02 * AbilityPower / 100) * EnemyMaxHealth],
            )
            // The values based on stacks are the damage cap against monsters
            .take("Magic Damage", Q(Void))?
            .take("Increased Damage", W(Max))?
            .take("Magic Damage", W(Min))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Min))?
            .take("Total Single-Target Damage", R(Max))?
            .combos([
                &[Ability(Q(Void)), Ability(E(Void)), Ability(W(Max))],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max))],
                &[Ability(W(Max)), Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
