use super::*;

impl GeneratorV2 for Zed {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Physical Damage", Q(Max))?
            .take("Reduced Damage", Q(Min))?
            .take("Physical Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Max))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(Q(Max)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Max))],
            ])?
            .end()
    }
}
