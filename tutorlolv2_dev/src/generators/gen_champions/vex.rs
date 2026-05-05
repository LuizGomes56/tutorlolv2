use super::*;

impl Generator for Vex {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 2 */
                (1, _2), /* Description 2 [1] */
                (2, _3), /* Description 2 [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
