use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
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
            .end()
    }
}
