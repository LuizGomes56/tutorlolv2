use super::*;

impl Generator for Xayah {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage Per Hit", Q(Min))?
            .take("Reduced Damage per Hit", Q(_1Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Total Reduced Damage", Q(_1Max))?
            .take("Physical Damage Per Feather", E(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[Ability(R(Void)), Ability(E(Void)), Attack, Attack],
                &[Ability(Q(Max)), Attack, Ability(E(Void))],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Max)), Attack, Ability(E(Void))],
                &[Ability(R(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
