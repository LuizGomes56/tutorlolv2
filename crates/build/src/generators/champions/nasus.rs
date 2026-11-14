use super::*;

impl Generator for Nasus {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage Per Tick", E(_1))?
            .take("Magic Damage", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Magic Damage Per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
            ])?
            .end()
    }
}
