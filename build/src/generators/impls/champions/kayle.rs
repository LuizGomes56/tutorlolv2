use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Level 11 - Aflame */])
            .modify(P(Void), |_| {
                f![20 + 3 * (Level - 11) + 0.1 * BonusAd + 0.25 * AbilityPower]
            })?
            .comment(P(Void), "Exalted fire wave, available from level 11")?
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void), /* Bonus Magic Damage */
                    (1, _1),   /* Passive Damage */
                ],
            )
            .modify(E(Void), |_| {
                f![(0.08 + 0.005 * (ELevel - 1) + 0.00015 * AbilityPower) * EnemyMissingHealth]
            })?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([Ability(Q(Void)), Attack, Ability(E(Void))])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([Attack, Ability(E(Void)), Ability(Q(Void))])?
            .combo([Ability(Q(Void)), Attack, Attack, Ability(E(Void))])?
            .end()
    }
}
