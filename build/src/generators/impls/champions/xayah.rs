use super::*;

impl Generator for Xayah {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, Min),   /* Physical Damage Per Hit */
                    (1, _1Min), /* Reduced Damage per Hit */
                    (2, Max),   /* Total Physical Damage */
                    (3, _1Max), /* Total Reduced Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Physical Damage Per Feather */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .end()
    }
}
