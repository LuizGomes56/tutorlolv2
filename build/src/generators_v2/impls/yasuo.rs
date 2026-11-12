use super::*;

impl GeneratorV2 for Yasuo {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Min))?
            .take("Critical Strike Damage", Q(Max))?
            .take("Bonus Damage per Stack", E(_1Min))?
            .take("Magic Damage", E(Min))?
            .take("Maximum Bonus Damage", E(_1Max))?
            .take("Total Combined Damage", E(Max))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Min)),
                    Ability(E(Min)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                ],
                &[Ability(E(Min)), Attack, Ability(Q(Max)), Ability(E(Min))],
            ])?
            .end()
    }
}
