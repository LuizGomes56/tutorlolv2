use super::*;

impl Generator for Garen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(Key::W, [(0, _1) /* Damage Reduction */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Critical Strike Damage Per Spin */
                    (1, _2), /* IE Damage Per Spin */
                    (2, _3), /* Increased  IE Damage Per Spin */
                    (3, _4), /* Increased Critical Strike Damage Per Spin */
                    (4, _5), /* Increased Damage Per Spin */
                    (5, _6), /* Physical Damage Per Spin */
                ],
            )
            .ability(Key::R, [(0, _1) /* True Damage */])
            .end()
    }
}
