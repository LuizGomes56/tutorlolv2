use super::*;

impl Generator for KSante {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* All Out Bonus */
                (1, _2), /* Innate */
                (2, _3), /* Innate [1] */
                (3, _4), /* Innate [2] */
                (4, _5), /* Innate [3] */
                (5, _6), /* Innate [4] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Physical Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Maximum Bonus True Damage */
                (1, _2), /* Minimum Bonus True Damage */
                (3, _3), /* Physical Damage */
                (4, _4), /* Total Maximum Mixed Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Physical Damage */
                (2, _2), /* Strike Physical Damage */
                (3, _3), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
