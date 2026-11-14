use super::*;

impl Generator for Gragas {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Magic Damage", Q(Min))?
            .take("Maximum Magic Damage", Q(Max))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[Ability(E(Void)), Ability(R(Void))],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Max)), Ability(W(Void))],
            ])?
            .end()
    }
}
