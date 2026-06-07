use super::*;

impl Generator for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Area Physical Damage */
                (1, _1),   /* True Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, Max),  /* Increased/Explosion Magic Damage */
                (1, Void), /* Magic Damage */
                (2, Min),  /* Subsequent Explosion Damage */
            ],
        )
        .ability(Key::R, [(4, Void) /* Magic Damage */])
        .end()
    }
}
