use super::*;

impl Generator for Kennen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Bonus Magic Damage */
                    (1, _2), /* Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (1, _1), /* Magic Damage */
                    (2, _2), /* Non-Champion Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, _1), /* Magic Damage Per Bolt */
                    (2, _2), /* Total Single-Target Damage */
                ],
            )
            .end()
    }
}
