use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 2 */
                (2, _3), /* Innate */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
