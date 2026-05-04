use super::*;

impl Generator for Galio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage Reduction */
                    (2, _2), /* Maximum Magic Damage */
                    (3, _3), /* Minimum Magic Damage */
                    (4, _4), /* Physical Damage Reduction */
                ],
            )
            .ability_nth(
                1,
                Key::W,
                [
                    (0, _5), /* Magic Damage Reduction */
                    (2, _6), /* Maximum Magic Damage */
                    (3, _7), /* Minimum Magic Damage */
                    (4, _8), /* Physical Damage Reduction */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Non-Champion Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
