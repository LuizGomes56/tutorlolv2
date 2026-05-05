use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Description 1 */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Damage Per Tick */
                    (1, _2), /* Maximum Damage */
                ],
            )
            .end()
    }
}
