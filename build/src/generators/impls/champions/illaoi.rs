use super::*;

impl Generator for Illaoi {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Description 1 [1] */
                (2, _3), /* Description 1 [2] */
            ],
        )
        .ability(Key::Q, [(0, Void) /* Damage Increase */])
        .ability(
            Key::W,
            [
                (0, Max), /* Additional Physical Damage */
                (1, Min), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Damage Transmission */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
