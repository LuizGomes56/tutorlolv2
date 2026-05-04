use super::*;

impl Generator for Nami {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (1, _1), /* Magic Damage */
                    (2, _2), /* Minimum Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Magic Damage Per Hit */
                    (2, _2), /* Total Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
