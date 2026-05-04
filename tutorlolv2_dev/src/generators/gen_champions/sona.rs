use super::*;

impl Generator for Sona {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Minimum Damage Mitigated */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
