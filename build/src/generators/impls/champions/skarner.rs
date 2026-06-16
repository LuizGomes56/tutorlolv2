use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .modify(P(Void), |dmg| f![dmg * EnemyMaxHealth])?
            .ability(
                Key::Q,
                [
                    (1, Min), /* Bonus Physical Damage per Hit */
                    (3, Max), /* Total Bonus Physical Damage */
                ],
            )
            .ability_nth(1, Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
