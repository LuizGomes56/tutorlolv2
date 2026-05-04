use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Minimum Damage */
                (1, _2), /* Physical Damage */
                (2, _3), /* Total Maximum Champion Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (1, _1), /* Bounce Critical Damage */
                (2, _2), /* Bounce Damage */
            ],
        )
        .end()
    }
}
