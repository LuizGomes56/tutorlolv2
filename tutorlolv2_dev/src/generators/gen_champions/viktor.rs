use super::*;

impl Generator for Viktor {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Modified Magic Damage */
                (2, _3), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Magic Damage Per Tick */
                (2, _3), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
