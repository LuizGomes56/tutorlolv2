use super::*;

impl Generator for Xerath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Increased Damage */
                    (2, Min), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Void), /* Increased Damage per Stack */
                    (2, Min),  /* Magic Damage */
                    (5, Max),  /* Total Magic Damage */
                ],
            )
            .end()
    }
}
