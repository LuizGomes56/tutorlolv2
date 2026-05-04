use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Physical Damage */
                (1, _2), /* Total Bonus Physical Damage */
            ],
        )
        .ability_nth(1, Key::Q, [(0, _3) /* Magic Damage */])
        .ability_nth(1, Key::W, [(0, _1) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* True Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
