use super::*;

impl Generator for Cassiopeia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Magic Damage Per Tick */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage Per Second */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Magic Damage */
                (3, _2), /* Total Enhanced Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
