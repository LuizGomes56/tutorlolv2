use super::*;

impl GeneratorV2 for Cassiopeia {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage Per Tick", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage Per Second", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Bonus Magic Damage", E(Min))?
            .take("Total Enhanced Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(R(Void)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                ],
            ])?
            .end()
    }
}
