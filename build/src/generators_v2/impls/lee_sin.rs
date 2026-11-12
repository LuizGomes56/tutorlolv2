use super::*;

impl GeneratorV2 for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(_1))?
            .take("Minimum Physical Damage", Q(Min))?
            .take("Maximum Physical Damage", Q(Max))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage", R(Min))?
            .take("Collision Physical Damage", R(Max))?
            .combos([
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(R(Max))],
                &[
                    Ability(Q(Min)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Min)), Ability(R(Max)), Ability(Q(Min)), Attack],
                &[
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
