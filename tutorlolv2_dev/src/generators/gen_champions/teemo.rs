use super::*;

impl Generator for Teemo {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void), /* Magic Damage On-Hit */
                    (1, Min),  /* Magic Damage per Tick */
                    (5, Max),  /* Total Poison Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage per Tick */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
