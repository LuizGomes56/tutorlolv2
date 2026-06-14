use super::*;

impl Generator for Viktor {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),  /* Magic Damage */
                (1, Void), /* Modified Magic Damage */
                (2, Max),  /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, Void), /* Magic Damage */
                (1, Min),  /* Magic Damage Per Tick */
                (2, Max),  /* Total Magic Damage */
            ],
        )
        .end()
    }
}
