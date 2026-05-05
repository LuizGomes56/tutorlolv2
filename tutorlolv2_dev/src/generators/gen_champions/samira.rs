use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 3 */
                (3, _2), /* Innate */
                (4, _3), /* Innate [1] */
                (5, _4), /* Innate [2] */
                (6, _5), /* Innate [3] */
                (7, _6), /* Innate [4] */
                (8, _7), /* Innate [5] */
                (9, _8), /* Innate [6] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Physical Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Physical Damage per Hit */
                (1, _2), /* Total Physical Damage */
            ],
        )
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (1, _1), /* Physical Damage Per Shot */
                (3, _2), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
