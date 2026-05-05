use super::*;

impl Generator for Hecarim {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */])
            .ability(Key::Q, [(1, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (2, _1), /* Magic Damage Per Tick */
                    (3, _2), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1), /* Maximum Physical Damage */
                    (1, _2), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::R, [(0, _1) /* Magic damage */])
            .end()
    }
}
