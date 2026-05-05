use super::*;

impl Generator for Warwick {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (1, _2), /* Innate [1] */
                (2, _3), /* Innate [2] */
                (3, _4), /* Innate [3] */
                (4, _5), /* Innate [4] */
                (5, _6), /* Innate [5] */
                (6, _7), /* Innate [6] */
            ],
        )
        .ability(Key::Q, [(2, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Damage Reduction */])
        .ability(Key::R, [(0, _1) /* Total Magic Damage */])
        .end()
    }
}
