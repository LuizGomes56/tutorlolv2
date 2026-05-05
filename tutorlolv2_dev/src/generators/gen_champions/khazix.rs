use super::*;

impl Generator for Khazix {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */, (1, _2) /* Innate [1] */])
            .ability(
                Key::Q,
                [
                    (0, _1), /* Isolated Target Physical Damage */
                    (1, _2), /* Physical Damage */
                ],
            )
            .ability(Key::W, [(1, _1) /* Physical Damage */])
            .ability(Key::E, [(0, _1) /* Physical Damage */])
            .end()
    }
}
