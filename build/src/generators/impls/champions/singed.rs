use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),  /* Magic Damage per Second */
                (1, Max),  /* Magic Damage per Tick */
                (2, Void), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .end()
    }
}
