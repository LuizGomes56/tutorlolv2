use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate */
                (2, _2), /* Swinging Kama */
                (3, _3), /* Swinging Kama [1] */
                (4, _4), /* Swinging Kama [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Maximum Magic Damage */
                (2, _3), /* Minimum Magic Damage */
            ],
        )
        .end()
    }
}
