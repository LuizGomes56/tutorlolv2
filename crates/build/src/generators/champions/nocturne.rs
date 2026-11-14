use super::*;

impl Generator for Nocturne {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .damage_type(P(Void), Physical)
            .comment(P(Void), "Empowered basic attack")
            .take("Physical damage", Q(Void))?
            .take("Magic Damage per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack, Attack],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
