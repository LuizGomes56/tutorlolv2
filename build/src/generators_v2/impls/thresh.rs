use super::*;

impl GeneratorV2 for Thresh {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Minimum Bonus Magic Damage", E(_1Min))?
            .take("Maximum Bonus Magic Damage", E(_1Max))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void)), Attack],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
