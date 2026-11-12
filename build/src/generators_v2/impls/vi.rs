use super::*;

impl GeneratorV2 for Vi {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Physical Damage", Q(Min))?
            .take("Maximum Physical Damage", Q(Max))?
            .take("Bonus Physical Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .damage_type(E(Void), Physical)
            .combos([
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Max)), Attack, Ability(E(Void)), Attack, Attack],
            ])?
            .end()
    }
}
