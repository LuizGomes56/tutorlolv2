use super::*;

impl Generator for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate */
                (2, _2), /* Innate [1] */
                (3, _3), /* Innate [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Reduced Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Increased Damage */
                (1, _4), /* Physical Damage */
                (2, _5), /* Reduced Damage */
                (3, _6), /* Subsequent Increased Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Bonus Magic Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(1, _1) /* Physical Damage */])
        .end()
    }
}
