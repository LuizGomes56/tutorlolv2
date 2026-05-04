use super::*;

impl Generator for Ahri {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Damage Per Pass */
                (1, _2), /* Total Mixed Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Primary Magic Damage */
                (2, _2), /* Subsequent Magic Damage */
                (4, _3), /* Total Single-Target Damage */
            ],
        )
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
