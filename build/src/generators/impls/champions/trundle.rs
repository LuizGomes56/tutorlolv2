use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Bonus Physical Damage */])
            .ability(
                Key::R,
                [
                    (2, Min),  /* Initial Magic Damage */
                    (3, Void), /* Magic Damage Per Second */
                    (5, Max),  /* Total Magic Damage */
                ],
            )
            .end()
    }
}
