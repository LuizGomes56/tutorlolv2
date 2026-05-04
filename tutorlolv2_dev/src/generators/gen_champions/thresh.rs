use super::*;

impl Generator for Thresh {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability_nth(1, Key::Q, [(0, _2) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Maximum Bonus Magic Damage */
                    (2, _3), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability_nth(
                1,
                Key::E,
                [
                    (0, _4), /* Magic Damage */
                    (1, _5), /* Maximum Bonus Magic Damage */
                    (2, _6), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability_nth(
                2,
                Key::E,
                [
                    (0, _7),    /* Magic Damage */
                    (1, _8),    /* Maximum Bonus Magic Damage */
                    (2, _1Min), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability_nth(
                3,
                Key::E,
                [
                    (0, _2Min), /* Magic Damage */
                    (1, _3Min), /* Maximum Bonus Magic Damage */
                    (2, _4Min), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability_nth(
                4,
                Key::E,
                [
                    (0, _5Min), /* Magic Damage */
                    (1, _6Min), /* Maximum Bonus Magic Damage */
                    (2, _7Min), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
