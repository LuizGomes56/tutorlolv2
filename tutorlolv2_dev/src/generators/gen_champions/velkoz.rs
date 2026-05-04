use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability_nth(1, Key::Q, [(0, _2) /* Magic Damage */])
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
            .ability_nth(
                1,
                Key::R,
                [
                    (0, _3), /* Damage Per Tick */
                    (1, _4), /* Maximum Damage */
                ],
            )
            .end()
    }
}
