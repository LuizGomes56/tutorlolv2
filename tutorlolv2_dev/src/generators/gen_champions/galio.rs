use super::*;

impl Generator for Galio {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Description 2 */
                (2, _2), /* Innate */
                (3, _3), /* Innate [1] */
                (4, _4), /* Innate [2] */
                (5, _5), /* Innate [3] */
                (6, _6), /* Innate [4] */
                (7, _7), /* Innate [5] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage Reduction */
                (2, _2), /* Maximum Magic Damage */
                (3, _3), /* Minimum Magic Damage */
                (4, _4), /* Physical Damage Reduction */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Non-Champion Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
