use super::*;

impl Generator for Aatrox {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* First Cast Damage */
                (1, _2), /* First Sweetspot Damage */
                (4, _3), /* Second Cast Damage */
                (5, _4), /* Second Sweetspot Damage */
                (6, _5), /* Third Cast Damage */
                (7, _6), /* Third Sweetspot Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (1, _1), /* Physical Damage */
                (3, _2), /* Total Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Bonus Attack Damage */])
        .end()
    }
}
