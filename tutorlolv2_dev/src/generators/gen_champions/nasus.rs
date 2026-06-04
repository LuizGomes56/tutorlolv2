use super::*;

impl Generator for Nasus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::E,
                [
                    (1, Min), /* Magic Damage */
                    (2, _1),  /* Magic Damage Per Tick */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (3, Min), /* Magic Damage Per Tick */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
