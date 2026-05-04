use super::*;

impl Generator for Karma {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, _2), /* Bonus Magic Damage */
                    (1, _3), /* Magic Damage */
                    (2, _4), /* Total Bonus Damage */
                    (3, _5), /* Total Damage */
                    (4, _6), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage */
                    (2, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
