use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Increased Bonus Damage */
                (4, _3), /* Total Increased Damage */
                (5, _4), /* Total Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _5), /* Bonus Magic Damage */
                (1, _6), /* Increased Bonus Damage */
                (4, _7), /* Total Increased Damage */
                (5, _8), /* Total Magic Damage */
            ],
        )
        .ability_nth(
            2,
            Key::Q,
            [
                (0, _1Min), /* Bonus Magic Damage */
                (1, _2Min), /* Increased Bonus Damage */
                (4, _3Min), /* Total Increased Damage */
                (5, _4Min), /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .end()
    }
}
