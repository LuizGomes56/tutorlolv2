use super::*;

impl Generator for Hwei {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */])
            .ability_nth(2, Key::Q, [(0, _2) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability_nth(
                1,
                Key::W,
                [
                    (0, _2), /* Magic Damage */
                    (1, _3), /* Maximum Damage */
                    (2, _4), /* Maximum Damage Increase */
                ],
            )
            .ability_nth(1, Key::E, [(0, _1) /* Magic Damage */])
            .ability_nth(
                2,
                Key::E,
                [
                    (0, _2), /* Magic Damage */
                    (1, _3), /* Magic Damage per Tick */
                    (2, _4), /* Total Fissure Magic Damage */
                ],
            )
            .ability_nth(
                3,
                Key::E,
                [
                    (0, _5), /* Bonus Magic Damage */
                    (2, _6), /* Maximum Magic Damage */
                    (3, _7), /* Reduced Bonus Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Magic Damage per Tick */
                    (2, _3), /* Maximum Total Damage */
                    (3, _4), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
