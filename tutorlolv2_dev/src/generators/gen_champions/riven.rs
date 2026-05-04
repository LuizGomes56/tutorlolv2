use super::*;

impl Generator for Riven {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Physical Damage */])
        .ability_nth(
            1,
            Key::R,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .end()
    }
}
