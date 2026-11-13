use super::*;

impl Generator for Graves {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Physical Damage", 1, Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", R(Void))?
            .take("Reduced Damage", R(_1))?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Max)), Attack, Ability(R(Void))],
                &[Ability(W(Void)), Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
                &[Attack, Attack, Ability(Q(Max)), Attack],
                &[Ability(W(Void)), Attack, Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
