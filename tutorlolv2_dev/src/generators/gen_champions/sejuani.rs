use super::*;

impl Generator for Sejuani {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Physical Damage */
                    (1, _2), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Increased Damage */
                    (1, _2), /* Magic Damage */
                ],
            )
            .end()
    }
}
