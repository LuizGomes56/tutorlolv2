use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Sion {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 3, P(Void))?
            .damage_type(P(Void), Physical)
            .take("Minimum Physical Damage", Q(Min))?
            .take("Maximum Physical Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Minimum Physical Damage", R(Min))?
            .take("Maximum Physical Damage", R(Max))?
            .comment(P(Void), "Glory in Death bonus damage on-hit")
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
