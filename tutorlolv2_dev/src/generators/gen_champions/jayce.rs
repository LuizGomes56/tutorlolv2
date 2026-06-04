use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, Max), /* Increased Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .ability_nth(
                1,
                Key::W,
                [
                    (0, _1Min), /* Physical Damage */
                    (1, _1Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .end()
    }
}
