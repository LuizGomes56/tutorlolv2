use super::*;

impl Generator for Viego {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Bonus Physical Damage */
                (1, Min),  /* Minimum Bonus Damage */
                (2, Max),  /* Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
