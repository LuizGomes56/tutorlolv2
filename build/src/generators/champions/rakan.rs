use super::*;

impl Generator for Rakan {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(R(Void)), Ability(W(Void)), Ability(Q(Void))],
                &[Ability(R(Void)), Ability(W(Void)), Attack, Ability(Q(Void))],
                &[Ability(W(Void)), Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Ability(W(Void)), Attack],
                &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
            ])?
            .end()
    }
}
