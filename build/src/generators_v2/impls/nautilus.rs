use super::*;

impl GeneratorV2 for Nautilus {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage per Instance", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Magic Damage", E(Min))?
            .take("Maximum Total Damage", E(Max))?
            .take("Magic Damage", R(Min))?
            .take("Increased Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Max))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
