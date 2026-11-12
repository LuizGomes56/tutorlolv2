use super::*;

impl GeneratorV2 for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take_nth("Magic Damage", 1, W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Damage Per Tick", R(Min))?
            .take("Maximum Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Min)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(E(Void)),
                    Ability(W(Min)),
                ],
                &[Ability(E(Void)), Ability(W(Max))],
            ])?
            .end()
    }
}
