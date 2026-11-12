use super::*;

impl GeneratorV2 for Darius {
    fn generate(&mut self) -> MayFail {
        // Total per stack, then per tick, the total at 5 stacks and its tick
        self.take_nth("Hemorrhage", 0, P(Min))?
            .comment(P(Min), "Hemorrhage total damage per stack")
            .take_nth("Hemorrhage", 2, P(Max))?
            .comment(P(Max), "Hemorrhage total damage at 5 stacks")
            .take("Physical Damage (Blade)", Q(Max))?
            .take("Reduced Damage (Handle)", Q(Min))?
            .take("Bonus Physical Damage", W(Void))?
            .take("True Damage", R(Min))?
            .take("Maximum True Damage", R(Max))?
            .combos([
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(R(Min)),
                ],
                &[Ability(Q(Max)), Attack, Ability(W(Void)), Ability(R(Max))],
                &[Attack, Ability(W(Void)), Ability(Q(Max))],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max))],
                &[Ability(Q(Max)), Attack, Ability(W(Void))],
            ])?
            .end()
    }
}
