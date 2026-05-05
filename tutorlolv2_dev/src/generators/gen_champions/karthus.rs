use super::*;

impl Generator for Karthus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [(0, _1) /* Description 1 */, (1, _2) /* Innate */],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Isolated Enhanced Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Damage Per Second */
                (1, _2), /* Magic Damage Per Tick */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
