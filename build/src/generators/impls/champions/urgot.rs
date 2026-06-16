use super::*;

impl Generator for Urgot {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .modify(P(Void), |dmg| {
                if let Ok(hp_scaling) = Scaling::based_on_level_raw(
                    Level,
                    [
                        LevelArm::new(..4, 0.02),
                        LevelArm::new(4..=7, 0.03),
                        LevelArm::new(7..=10, 0.04),
                        LevelArm::new(10..=13, 0.05),
                        LevelArm::new(13.., 0.06),
                    ],
                )
                .render(Level)
                {
                    f![(dmg * AttackDamage) + hp_scaling * EnemyMaxHealth]
                } else {
                    core::hint::cold_path();
                    f![(dmg)]
                }
            })?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Modified Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .end()
    }
}
