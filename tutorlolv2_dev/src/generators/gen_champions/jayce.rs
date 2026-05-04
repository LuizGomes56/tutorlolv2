use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, _2), /* Increased Damage */
                    (1, _3), /* Physical Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage Per Tick */
                    (2, _2), /* Total Magic Damage */
                ],
            )
            .ability_nth(
                1,
                Key::W,
                [
                    (0, _3), /* Physical Damage */
                    (1, _4), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(1, _1) /* Magic Damage */])
            .end()
    }
}
