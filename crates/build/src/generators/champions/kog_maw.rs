use super::*;

impl Generator for KogMaw {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Magic Damage", Q(Void))?
            .map_change(Q(Void), "AP ratio")?
            .take("Bonus Magic Damage", W(Void))?
            .map_change(W(Void), "Base damage")?
            .map_change(W(Void), "Target health ratio")?
            .take("Magic Damage", E(Void))?
            .map_change(E(Void), "AP ratio")?
            .take("Minimum Magic Damage", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Min)),
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
