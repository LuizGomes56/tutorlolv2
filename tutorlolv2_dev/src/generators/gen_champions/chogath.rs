use super::*;

impl Generator for Chogath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic damage */])
            .ability(Key::W, [(0, _1) /* Magic damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (3, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (3, _1), /* Champion True Damage */
                    (4, _2), /* Non-Champion True Damage */
                ],
            )
            .end()
    }
}
