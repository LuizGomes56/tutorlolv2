use super::*;

impl Generator for Xerath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Increased Damage */
                    (2, _2), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, _1), /* Increased Damage per Stack */
                    (2, _2), /* Magic Damage */
                    (5, _3), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
