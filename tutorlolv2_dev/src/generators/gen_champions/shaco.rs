use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (2, _1), /* Increased Damage */
                    (4, _2), /* Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Increased Damage */
                    (1, _2), /* Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Increased Modified Damage */
                    (1, _2), /* Magic Damage */
                    (2, _3), /* Modified Magic Damage */
                ],
            )
            .ability_nth(
                1,
                Key::R,
                [
                    (0, _4), /* Increased Modified Damage */
                    (1, _5), /* Magic Damage */
                    (2, _6), /* Modified Magic Damage */
                ],
            )
            .end()
    }
}
