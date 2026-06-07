use super::*;

impl Generator for Morgana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1Max), /* Maximum Damage Per Tick */
                    (1, Max),   /* Maximum Total Damage */
                    (2, _1Min), /* Minimum Damage Per Tick */
                    (3, Min),   /* Minimum Total Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
