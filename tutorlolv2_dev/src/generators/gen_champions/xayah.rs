use super::*;

impl Generator for Xayah {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, _1), /* Physical Damage Per Hit */
                    (1, _2), /* Reduced Damage per Hit */
                    (2, _3), /* Total Physical Damage */
                    (3, _4), /* Total Reduced Damage */
                ],
            )
            .ability(Key::E, [(1, _1) /* Physical Damage Per Feather */])
            .ability(Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
