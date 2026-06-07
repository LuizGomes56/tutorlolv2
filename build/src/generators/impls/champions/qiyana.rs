use super::*;

impl Generator for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, Max),   /* Increased Damage */
                    (1, _1Max), /* Physical Damage */
                    (2, _1Min), /* Reduced Damage */
                    (3, Min),   /* Subsequent Increased Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(1, Void) /* Physical Damage */])
            .end()
    }
}
