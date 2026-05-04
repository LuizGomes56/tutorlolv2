use super::*;

impl Generator for Kayn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Non-Champion Damage */
                (2, _2), /* Physical Damage */
                (4, _3), /* Total Non-Champion Damage */
                (5, _4), /* Total Physical Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (1, _5), /* Non-Champion Damage */
                (2, _6), /* Physical Damage */
                (4, _7), /* Total Non-Champion Damage */
                (5, _8), /* Total Physical Damage */
            ],
        )
        .ability_nth(
            2,
            Key::Q,
            [
                (1, _1Min), /* Non-Champion Damage */
                (2, _2Min), /* Physical Damage */
                (4, _3Min), /* Total Non-Champion Damage */
                (5, _4Min), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Physical Damage */])
        .ability_nth(1, Key::W, [(0, _2) /* Physical Damage */])
        .ability_nth(2, Key::W, [(0, _3) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .ability_nth(1, Key::R, [(0, _2) /* Physical Damage */])
        .ability_nth(2, Key::R, [(0, _3) /* Physical Damage */])
        .ability_nth(3, Key::R, [(0, _4) /* Physical Damage */])
        .end()
    }
}
