use super::*;

impl GeneratorV2 for Nami {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Minimum Damage", W(Min))?
            .take("Magic Damage", W(Max))?
            .take("Bonus Magic Damage Per Hit", E(Min))?
            .take("Total Bonus Magic Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max)), Ability(E(Max)), Attack],
                &[Ability(E(Max)), Ability(W(Max)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
