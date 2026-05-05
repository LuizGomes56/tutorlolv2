use super::*;

impl Generator for Sett {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 2 */
                (1, _2), /* Description 2 [1] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Physical Damage */
                (1, _2), /* Total Bonus Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Reduced Damage */
            ],
        )
        .end()
    }
}
