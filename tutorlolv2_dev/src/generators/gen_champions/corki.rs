use super::*;

impl Generator for Corki {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Magic Damage Per Tick */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Physical Damage Per Tick */
                    (2, _2), /* Total Physical Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Big One Physical Damage */
                    (1, _2), /* Physical Damage */
                ],
            )
            .end()
    }
}
