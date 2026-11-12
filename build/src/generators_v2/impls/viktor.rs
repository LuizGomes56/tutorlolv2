use super::*;

impl GeneratorV2 for Viktor {
    fn generate(&mut self) -> MayFail {
        self.take("Modified Magic Damage", Q(Void))?
            .take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take_nth("Magic Damage", 1, E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .take("Magic Damage Per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(E(Max))],
            ])?
            .end()
    }
}
