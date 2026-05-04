use super::*;

impl Generator for Draven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(Key::E, [(0, _1) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Minimum Physical Damage */
                    (1, _2), /* Minimum Total Damage */
                    (2, _3), /* Physical Damage */
                    (3, _4), /* Total Physical Damage */
                ],
            )
            .end()
    }
}
