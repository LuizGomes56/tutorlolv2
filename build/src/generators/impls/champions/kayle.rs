use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Level 11 - Aflame", Void)])
            .modify(P(Void), |_| {
                f![20 + 3 * (Level - 11) + 0.1 * BonusAd + 0.25 * AbilityPower]
            })?
            .comment(P(Void), "Exalted fire wave, available from level 11")?
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Bonus Magic Damage", Void),
                    ("Passive Damage", _1),
                ],
            )
            .modify(E(Void), |_| {
                f![(0.08 + 0.005 * (ELevel - 1) + 0.00015 * AbilityPower) * EnemyMissingHealth]
            })?
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Attack, Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
