use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Damage Per Tick */
                    (1, Max), /* Maximum Damage */
                ],
            )
            .end()
    }
}
