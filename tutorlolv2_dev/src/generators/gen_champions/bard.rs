use super::*;

impl Generator for Bard {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Innate - Meeps */
                (3, _2), /* Innate - Meeps [1] */
                (4, _3), /* Innate - Meeps [2] */
                (5, _4), /* Innate - Meeps [3] */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Magic Damage */])
        .end()
    }
}
