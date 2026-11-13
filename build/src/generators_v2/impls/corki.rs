use super::*;

impl GeneratorV2 for Corki {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .damage_type(P(Void), True)
            .comment(P(Void), "Bonus true damage per basic attack")
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage Per Tick", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Physical Damage Per Tick", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .take("Physical Damage", R(Min))?
            .take("Big One Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Attack,
                    Ability(R(Min)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Min)),
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
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack],
            ])?
            .end()
    }
}
