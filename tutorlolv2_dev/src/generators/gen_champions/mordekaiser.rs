use super::*;

impl Generator for Mordekaiser {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Isolated Damage Increase */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
