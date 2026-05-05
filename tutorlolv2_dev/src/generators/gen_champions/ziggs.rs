use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(1, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage per Mine */
                    (1, _2), /* Maximum Total Magic Damage */
                    (2, _3), /* Reduced Damage per Mine */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Epicenter Magic Damage */
                    (1, _2), /* Reduced Damage */
                ],
            )
            .end()
    }
}
