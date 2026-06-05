use super::*;

impl Generator for Sett {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 2 */])
            .ability(
                Key::Q,
                [
                    (0, Min), /* Bonus Physical Damage */
                    (1, Max), /* Total Bonus Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .end()
    }
}
