use super::*;

impl Generator for MissFortune {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate (Inconsistent) */])
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
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
                    (0, Max), /* Maximum Total Physical Damage */
                    (1, Min), /* Physical Damage per Wave */
                ],
            )
            .end()
    }
}
