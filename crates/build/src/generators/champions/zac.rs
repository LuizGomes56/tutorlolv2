use super::*;

impl Generator for Zac {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Reduced Damage Per Hit", R(Void))?
            .take("Magic Damage Per Hit", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(R(Void)), Ability(W(Void))],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
