use super::*;

impl Generator for Kennen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1),   /* Bonus Magic Damage */
                    (1, Void), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage Per Bolt */
                    (2, Max), /* Total Single-Target Damage */
                ],
            )
            .end()
    }
}
