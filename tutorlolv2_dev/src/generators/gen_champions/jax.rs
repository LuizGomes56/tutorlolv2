use super::*;

impl Generator for Jax {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::W, [(0, _1) /* Additional Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Maximum Magic Damage */
                    (1, _2), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Additional Magic Damage */
                    (5, _2), /* Magic Damage */
                ],
            )
            .end()
    }
}
