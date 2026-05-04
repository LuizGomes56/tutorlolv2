use super::*;

impl Generator for Nautilus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage per Instance */
                    (2, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Maximum Total Damage */
                    (3, _3), /* Reduced Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Increased Damage */
                    (2, _2), /* Magic Damage */
                ],
            )
            .ability_nth(
                1,
                Key::R,
                [
                    (0, _3), /* Increased Damage */
                    (2, _4), /* Magic Damage */
                ],
            )
            .end()
    }
}
