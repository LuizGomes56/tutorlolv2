use super::*;

impl Generator for Yorick {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(Key::E, [(3, Void) /* Magic Damage */])
            .modify(E(Void), |_| {
                f![(0.06 + 0.005 * (ELevel - 1) + 0.0003 * AbilityPower) * EnemyMaxHealth]
            })?
            .insert(
                P(Void),
                Ability {
                    name: "Mist Walker".into(),
                    damage_type: Physical,
                    attributes: Undefined,
                    comment: "Pet: Mist Walker basic attack".into(),
                    damage: f![15
                        + 5 * (Level - 1) * (0.7025 + 0.0175 * (Level - 1))
                        + 0.2 * BonusAd],
                },
            )
            .insert(
                R(Void),
                Ability {
                    name: "Maiden of the Mist".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Maiden of the Mist basic attack".into(),
                    damage: f![50 + 25 * (RLevel - 1) + 0.3 * BonusAd],
                },
            )
            .combo([Attack, Ability(E(Void)), Attack, Ability(Q(Void))])?
            .combo([Ability(E(Void)), Attack, Ability(Q(Void)), Attack])?
            .combo([Attack, Ability(Q(Void)), Ability(E(Void))])?
            .end()
    }
}
