use super::*;

impl Generator for Ezreal {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::W, [(0, _1) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Modified Damage */
                ],
            )
            .end()
    }
}
