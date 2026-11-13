use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        let per_stack = self.value_nth("Innate - Caustic Wounds", 1, Key::P)?;

        self.take_nth("Innate - Caustic Wounds", 0, P(Void))?
            .take_nth("Innate - Caustic Wounds", 3, P(_1))?
            .edit(P(Void), |p| f![p + Stacks * (per_stack)])
            .comment(
                P(Void),
                "Caustic Wounds on-hit damage, Stacks = Plasma stacks",
            )
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
