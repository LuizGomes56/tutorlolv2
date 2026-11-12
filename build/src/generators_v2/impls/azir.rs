use super::*;

impl GeneratorV2 for Azir {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .comment(W(Void), "Sand Soldier attack")
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
