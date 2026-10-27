use super::*;

impl Generator for Irelia {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Ionian Fervor */])
            .modify(P(Void), |_| f![10 + 51 / 17 * (Level - 1) + 0.2 * BonusAd])?
            .comment(P(Void), "Bonus magic damage on-hit at maximum stacks")?
            .ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Max)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
