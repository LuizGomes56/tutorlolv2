use super::*;

impl Generator for Zed {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Reduced Damage */
            ],
        )
        .ability(Key::E, [(1, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
