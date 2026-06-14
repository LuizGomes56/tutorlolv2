use super::*;

impl Generator for Nunu {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Champion Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (2, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1),    /* Magic Damage (Root) */
                    (1, Min),   /* Magic Damage Per Hit */
                    (2, Max),   /* Maximum Total Magic Damage */
                    (4, _1Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Maximum Magic Damage */])
            .end()
    }
}
