use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage", W(Min))?
            .take("Increased Damage", W(Max))?
            .take("Magic Damage", E(Min))?
            .take("Increased Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .take("Modified Magic Damage", R(Min))?
            .take("Increased Modified Damage", R(Max))?
            .damage_type(Q(Void), Physical)
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(R(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack],
            ])?
            .end()
    }
}
