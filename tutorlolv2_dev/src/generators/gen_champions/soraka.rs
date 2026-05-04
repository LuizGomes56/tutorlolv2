use super::*;

impl Generator for Soraka {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (2, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
