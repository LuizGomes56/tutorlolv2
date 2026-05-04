use super::*;

impl Generator for Poppy {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, _1), /* Physical Damage */
                (5, _2), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Physical Damage */
                (2, _2), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Increased Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .end()
    }
}
