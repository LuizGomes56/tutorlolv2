use super::*;

impl Generator for Aurora {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, _1) /* Innate */, (4, _2) /* Innate [2] */])
            .merge_mul([P(_1), P(_2)], P(Void))?
            .ability(
                Key::Q,
                [
                    (0, _2),    /* Magic Damage */
                    (1, Max),   /* Maximum Magic Damage */
                    (2, Min),   /* Minimum Magic Damage */
                    (3, _1Max), /* Subsequent Bolt Maximum Magic Damage */
                    (4, _1Min), /* Subsequent Bolt Minimum Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
