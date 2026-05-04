use super::*;

impl Generator for Brand {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Increased Damage */
                    (1, _2), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage */
                    (2, _2), /* Total Single-Target Damage */
                ],
            )
            .end()
    }
}
