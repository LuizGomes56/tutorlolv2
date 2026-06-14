use super::*;

impl Generator for Aphelios {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Bonus Attack Damage */])
            .ability_nth(1, Key::P, [(1, _2) /* Description 1 */])
            .ability_nth(2, Key::P, [(0, _3) /* Description 0 */])
            .ability_nth(
                4,
                Key::P,
                [
                    (0, _4), /* Description 0 */
                    (1, _5), /* Description 0 [1] */
                ],
            )
            .ability_nth(
                5,
                Key::P,
                [
                    (2, _6), /* Description 2 */
                    (3, _7), /* Description 2 [1] */
                ],
            )
            .end()
    }
}
