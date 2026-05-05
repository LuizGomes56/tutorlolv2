use super::*;

impl Generator for Nocturne {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (2, _2), /* Innate */
                (3, _3), /* Innate [1] */
                (4, _4), /* Innate [2] */
                (5, _5), /* Innate [3] */
                (6, _6), /* Innate [4] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Attack Damage */
                (2, _2), /* Physical damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, _1), /* Magic Damage per Tick */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
