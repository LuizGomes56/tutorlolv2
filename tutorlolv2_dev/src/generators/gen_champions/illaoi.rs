use super::*;

impl Generator for Illaoi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 1 [1] */
                (2, _3), /* Description 1 [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Damage Increase */])
        .ability(
            Key::W,
            [
                (0, _1), /* Additional Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Damage Transmission */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
