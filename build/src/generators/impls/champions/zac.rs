use super::*;

impl Generator for Zac {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Min),  /* Magic Damage Per Hit */
                (1, Void), /* Reduced Damage Per Hit */
                (2, Max),  /* Total Magic Damage */
            ],
        )
        .end()
    }
}
