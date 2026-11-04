use super::*;

impl Generator for Irelia {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Ionian Fervor", Void)])
            .modify(P(Void), |_| f![10 + 51 / 17 * (Level - 1) + 0.2 * BonusAd])?
            .comment(P(Void), "Bonus magic damage on-hit at maximum stacks")?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
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
