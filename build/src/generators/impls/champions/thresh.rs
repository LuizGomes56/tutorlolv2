use super::*;

impl Generator for Thresh {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void),  /* Magic Damage */
                    (1, _1Max), /* Maximum Bonus Magic Damage */
                    (2, _1Min), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
