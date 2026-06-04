use super::*;

impl Generator for Jax {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Additional Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (1, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1),   /* Additional Magic Damage */
                    (5, Void), /* Magic Damage */
                ],
            )
            .end()
    }
}
