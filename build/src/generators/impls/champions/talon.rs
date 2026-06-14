use super::*;

impl Generator for Talon {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(
                Key::Q,
                [
                    (0, Max), /* Critical Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (0, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .end()
    }
}
