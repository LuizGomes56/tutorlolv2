use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Bonus Damage Per Second", Q(_1))?
            .take("Minimum Magic Damage", Q(Min))?
            .take("Maximum Magic Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .damage_type(P(Void), Magic)
            .combos([
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(W(Void))],
                &[Ability(Q(Max)), Ability(W(Void))],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Min)), Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
