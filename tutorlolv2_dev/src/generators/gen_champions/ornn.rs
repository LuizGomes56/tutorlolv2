use super::*;

impl Generator for Ornn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage Per Tick */
                    (3, _2), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage */
                    (2, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
