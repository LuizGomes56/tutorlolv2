use super::*;

impl Generator for Brand {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Ablaze */
                (1, _2), /* Ablaze [1] */
                (2, _3), /* Ablaze [2] */
                (3, _4), /* Description 2 */
                (4, _5), /* Description 2 [1] */
                (5, _6), /* Description 2 [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Increased Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage */
                (2, _2), /* Total Single-Target Damage */
            ],
        )
        .end()
    }
}
