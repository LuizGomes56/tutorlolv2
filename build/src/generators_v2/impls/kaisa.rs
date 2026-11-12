use super::*;

impl GeneratorV2 for Kaisa {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Caustic Wounds", 0, P(Void))?
            .take_nth("Innate - Caustic Wounds", 0, P(_1))?
            .set(P(Void), {
                f![4 + 26 / 17 * (Level - 1)
                    + 0.12 * AbilityPower
                    + Stacks * (1 + 7 / 17 * (Level - 1) + 0.03 * AbilityPower)]
            })
            .comment(
                P(Void),
                "Caustic Wounds on-hit damage, Stacks = Plasma stacks",
            )
            .set(P(_1), {
                f![(0.15 + 0.0006 * AbilityPower) * EnemyMissingHealthFlat]
            })
            .comment(P(_1), "Plasma detonation at 5 stacks")
            .take("Physical Damage Per Missile", Q(Min))?
            .take("Total Single-Target Damage", Q(Max))?
            .take("Total Evolved Single-Target Damage", Q(_1))?
            .take("Magic Damage", W(Void))?
            .combos([
                &[Ability(W(Void)), Ability(Q(Max)), Attack, Attack, Attack],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
                &[Attack, Ability(Q(Max)), Attack, Ability(W(Void)), Attack],
                &[Attack, Ability(W(Void)), Ability(Q(Max)), Attack, Attack],
            ])?
            .end()
    }
}
