use super::*;

impl Generator for Nasus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(
                Key::E,
                [
                    (1, _1), /* Magic Damage */
                    (2, _2), /* Magic Damage Per Tick */
                    (3, _3), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (3, _1), /* Magic Damage Per Tick */
                    (4, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
