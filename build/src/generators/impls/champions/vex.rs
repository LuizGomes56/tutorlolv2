use super::*;

impl Generator for Vex {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 2 */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
