use super::*;

impl Generator for Thresh {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Maximum Bonus Magic Damage */
                    (2, _3), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
