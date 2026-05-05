use super::*;

impl Generator for Urgot {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 1 [1] */
                (2, _3), /* Description 1 [2] */
                (3, _4), /* Description 1 [3] */
                (4, _5), /* Description 1 [4] */
                (5, _6), /* Description 1 [5] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Physical Damage */])
        .ability(Key::W, [(0, _1) /* Modified Physical Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
