use super::*;

impl Generator for Ashe {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (3, _2), /* Innate [1] */
                (4, _3), /* Innate [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1), /* Physical Damage Per Arrow */
                (2, _2), /* Total Damage Per Flurry */
            ],
        )
        .ability(Key::W, [(1, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
