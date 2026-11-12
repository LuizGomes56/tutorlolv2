use super::*;

impl GeneratorV2 for Rumble {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Overheated", 1, P(Void))?
            .comment(P(Void), "Overheated bonus magic damage on-hit")
            .take("Minimum Magic Damage", Q(Min))?
            .take("Maximum Magic Damage", Q(Max))?
            .take("Minimum Enhanced Damage", Q(_1Min))?
            .take("Maximum Enhanced Damage", Q(_1Max))?
            .take("Magic Damage", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Enhanced Damage", E(_1Min))?
            .take("Total Enhanced Damage", E(_1Max))?
            .take("Magic Damage per Second", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[Ability(E(Max)), Ability(Q(Max)), Attack, Attack, Attack],
                &[
                    Ability(Q(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
