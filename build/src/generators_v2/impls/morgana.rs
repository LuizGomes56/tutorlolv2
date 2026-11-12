use super::*;

impl GeneratorV2 for Morgana {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Minimum Total Damage", W(Min))?
            .take("Minimum Damage Per Tick", W(_1Min))?
            .take("Maximum Total Damage", W(Max))?
            .take("Maximum Damage Per Tick", W(_1Max))?
            .take("Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max))],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
