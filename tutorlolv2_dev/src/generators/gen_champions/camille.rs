use super::*;

impl Generator for Camille {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Bonus Physical Damage */
                (2, _2), /* Increased Mixed Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (1, _3), /* Bonus Physical Damage */
                (2, _4), /* Increased Mixed Damage */
            ],
        )
        .ability_nth(
            2,
            Key::Q,
            [
                (1, _5), /* Bonus Physical Damage */
                (2, _6), /* Increased Mixed Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (2, _1), /* Outer Cone Additional Damage */
                (3, _2), /* Physical Damage */
            ],
        )
        .ability_nth(1, Key::E, [(1, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Bonus Magic Damage */])
        .end()
    }
}
