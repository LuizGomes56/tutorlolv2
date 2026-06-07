use super::*;

impl Generator for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Collision Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .end()
    }
}
