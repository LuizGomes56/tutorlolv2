use super::*;

impl Generator for Bard {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Innate - Meeps */
                (4, _2), /* Innate - Meeps [2] */
            ],
        )
        .merge_sum([P(_1), P(_2)], P(Void))?
        .ability(Key::Q, [(1, Void) /* Magic Damage */])
        .end()
    }
}
