use super::*;

impl Generator for Orianna {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Min))?
            // At 2 stacks of Clockwork Winding
            .take_nth("Innate", 2, P(Max))?
            .take("Magic Damage", Q(Max))?
            .take("Reduced Damage", Q(Min))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(Q(Max)), Attack, Ability(W(Void))],
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(W(Void)), Attack],
                &[
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Max)), Ability(W(Void))],
                &[Ability(Q(Max)), Attack, Ability(W(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
