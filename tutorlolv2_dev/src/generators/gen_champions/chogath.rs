use super::*;

impl Generator for Chogath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic damage */])
            .ability(Key::W, [(0, Void) /* Magic damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(3, Void) /* Champion True Damage */])
            .end()
    }
}
