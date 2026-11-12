use super::*;

impl GeneratorV2 for Draven {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Physical Damage", E(Void))?
            .take("Minimum Total Damage", R(Min))?
            .take("Minimum Physical Damage", R(_1Min))?
            .take("Total Physical Damage", R(Max))?
            .take("Physical Damage", R(_1Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
