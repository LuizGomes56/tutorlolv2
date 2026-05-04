use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Magic Damage */
                    (1, _2), /* Passive Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
