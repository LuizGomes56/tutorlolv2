use super::*;

impl Generator for Nami {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (1, Max), /* Magic Damage */
                    (2, Min), /* Minimum Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Min), /* Bonus Magic Damage Per Hit */
                    (2, Max), /* Total Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
