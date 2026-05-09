use super::*;

impl Generator for Akshan {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (1, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Physical Damage per Shot */])
            .ability(
                Key::R,
                [
                    (1, _1Max), /* Damage to target on 67% missing hp */
                    (3, Max),   /* Maximum Physical Damage per Bullet */
                    (4, _1Min), /* Minimum Charged Physical Damage */
                    (5, Min),   /* Minimum Physical Damage per Bullet */
                ],
            )
            .end()
    }
}
