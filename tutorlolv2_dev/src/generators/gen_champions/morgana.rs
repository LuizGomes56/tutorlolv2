use super::*;

impl Generator for Morgana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Maximum Damage Per Tick */
                    (1, _2), /* Maximum Total Damage */
                    (2, _3), /* Minimum Damage Per Tick */
                    (3, _4), /* Minimum Total Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, _1), /* Magic Damage */
                    (3, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
