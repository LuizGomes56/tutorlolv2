use super::*;

impl Generator for Malzahar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
