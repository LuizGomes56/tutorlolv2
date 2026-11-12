use super::*;

impl GeneratorV2 for Evelynn {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Magic Damage", 0, Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Bonus Magic Damage", Q(_1))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Empowered Magic Damage", E(_1))?
            .take("Magic Damage", R(Min))?
            .take("Empowered Damage", R(Max))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
