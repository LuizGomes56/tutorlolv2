use super::*;

impl Generator for Zaahen {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),  /* Bonus Physical Damage */
                (3, Void), /* Physical Damage per Hit */
                (4, Max),  /* Total Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Min),  /* Initial Physical Damage */
                (1, Void), /* Subsequent Physical Damage */
                (2, Max),  /* Total Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Void), /* Bonus Magic Damage */
                (1, Max),  /* Increased Physical Damage */
                (2, Min),  /* Physical Damage */
            ],
        )
        .ability(Key::R, [(2, Void) /* Physical Damage */])
        .end()
    }
}
