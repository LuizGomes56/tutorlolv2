use super::*;

impl GeneratorV2 for Yorick {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .set(E(Void), {
                f![(0.06 + 0.005 * (ELevel - 1) + 0.0003 * AbilityPower) * EnemyMaxHealth]
            })
            .insert(
                P(Void),
                Damage {
                    name: "Mist Walker".into(),
                    damage_type: Physical,
                    attributes: Undefined,
                    comment: "Pet: Mist Walker basic attack".into(),
                    formula: f![15
                        + 5 * (Level - 1) * (0.7025 + 0.0175 * (Level - 1))
                        + 0.2 * BonusAd],
                },
            )?
            .insert(
                R(Void),
                Damage {
                    name: "Maiden of the Mist".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Maiden of the Mist basic attack".into(),
                    formula: f![50 + 25 * (RLevel - 1) + 0.3 * BonusAd],
                },
            )?
            .combos([
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
