use super::*;

impl Generator for Vayne {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Bonus True Damage */
                    (1, Min), /* Minimum Bonus Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Void), /* Bonus Physical Damage */
                    (1, Min),  /* Physical Damage */
                    (2, Max),  /* Total Physical Damage */
                ],
            )
            .end()
    }
}
