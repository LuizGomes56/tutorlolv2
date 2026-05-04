use super::*;

impl Generator for Lillia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Magic Damage */
                (3, _2), /* Total Mixed Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Increased Damage */
                (2, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
