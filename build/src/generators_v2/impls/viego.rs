use super::*;

impl GeneratorV2 for Viego {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Minimum Bonus Damage", Q(Min))?
            .take("Physical Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
                &[Ability(W(Void)), Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
