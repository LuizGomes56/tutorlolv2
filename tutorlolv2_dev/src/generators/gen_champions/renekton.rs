use super::*;

impl Generator for Renekton {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, _1), /* Enhanced Damage */
                (7, _2), /* Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Physical Damage Per Hit */
                (1, _2), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, _1), /* Enhanced Physical Damage */
                (2, _2), /* Physical Damage */
                (3, _3), /* Total Enhanced Damage */
                (4, _4), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Magic Damage Per Tick */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
