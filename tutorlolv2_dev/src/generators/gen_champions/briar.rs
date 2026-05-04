use super::*;

impl Generator for Briar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::W, [(2, _1) /* Physical Damage */])
            .ability_nth(
                1,
                Key::W,
                [
                    (0, _2), /* Bonus Physical Damage */
                    (2, _3), /* Non-Champion Bonus Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Magic Damage */
                    (3, _2), /* Maximum Magic Damage */
                    (4, _3), /* Minimum Magic Damage */
                    (5, _4), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(2, _1) /* Magic Damage */])
            .end()
    }
}
