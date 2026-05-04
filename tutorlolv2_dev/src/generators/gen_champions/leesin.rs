use super::*;

impl Generator for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, _2), /* Maximum Physical Damage */
                    (1, _3), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Collision Physical Damage */
                    (1, _2), /* Physical Damage */
                ],
            )
            .end()
    }
}
