use super::*;

impl Generator for Sion {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 2 */
                (2, _3), /* Innate */
                (3, _4), /* Innate [1] */
                (4, _5), /* Innate [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Maximum Base Damage Increase */
                (3, _2), /* Maximum Physical Damage */
                (6, _3), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .end()
    }
}
