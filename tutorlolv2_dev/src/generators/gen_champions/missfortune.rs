use super::*;

impl Generator for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage Per Tick */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Maximum Total Physical Damage */
                    (1, _2), /* Physical Damage per Wave */
                ],
            )
            .end()
    }
}
