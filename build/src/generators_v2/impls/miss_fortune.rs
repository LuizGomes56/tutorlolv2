use super::*;

impl GeneratorV2 for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            // 50% to 100% of AD by level, whose `% AD` the model drops
            .edit(P(Void), |p| f![p / 100 * AttackDamage])
            .comment(P(Void), "Love Tap bonus damage against unmarked enemies")
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage Per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Maximum Total Physical Damage", R(Max))?
            .take("Physical Damage per Wave", R(Min))?
            .combos([
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Max)), Ability(R(Max))],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
