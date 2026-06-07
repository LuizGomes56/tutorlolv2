use super::*;

impl Generator for Soraka {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
