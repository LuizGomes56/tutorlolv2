use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Maximum Magic Damage */
                    (2, _3), /* Minimum Magic Damage */
                ],
            )
            .end()
    }
}
