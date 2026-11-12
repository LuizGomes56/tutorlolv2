use super::*;

impl GeneratorV2 for Anivia {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Magic Damage", 1, Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", E(Min))?
            .take("Enhanced Damage", E(Max))?
            .take("Magic Damage per Tick", R(Min))?
            .take("Empowered Damage per Tick", R(Max))?
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Min)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[Ability(Q(Min)), Ability(E(Max)), Ability(Q(Min))],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
