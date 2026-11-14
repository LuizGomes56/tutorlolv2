use super::*;

impl Generator for Katarina {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Sinister Steel", 0, P(Void))?
            .comment(P(Void), "Dagger pickup slash")
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage Per Dagger", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .take("Physical Damage Per Dagger", R(_1Min))?
            .take("Maximum Physical Damage", R(_1Max))?
            .damage_type(R(_1Min), Physical)
            .damage_type(R(_1Max), Physical)
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
