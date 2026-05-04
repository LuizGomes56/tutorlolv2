use super::*;

impl Generator for Vayne {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Bonus True Damage */
                    (1, _2), /* Minimum Bonus Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Physical Damage */
                    (1, _2), /* Physical Damage */
                    (2, _3), /* Total Physical Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Bonus Attack Damage */])
            .end()
    }
}
