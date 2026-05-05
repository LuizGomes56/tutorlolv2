use super::*;

impl Generator for Talon {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 1 [1] */
                (2, _3), /* Description 1 [2] */
                (3, _4), /* Description 1 [3] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Critical Physical Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Physical Damage */
                (2, _2), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Physical Damage */
                (2, _2), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
