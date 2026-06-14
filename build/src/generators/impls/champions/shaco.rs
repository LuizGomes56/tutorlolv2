use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (2, Max), /* Increased Damage */
                    (4, Min), /* Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Max), /* Increased Damage */
                    (1, Min), /* Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max),  /* Increased Modified Damage */
                    (1, Void), /* Magic Damage */
                    (2, Min),  /* Modified Magic Damage */
                ],
            )
            .end()
    }
}
