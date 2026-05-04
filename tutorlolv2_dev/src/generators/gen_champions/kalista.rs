use super::*;

impl Generator for Kalista {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Bonus Magic Damage */
                    (1, _2), /* Maximum Non-Champion Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Damage per Additional Stack */
                    (2, _2), /* Physical Damage */
                ],
            )
            .end()
    }
}
