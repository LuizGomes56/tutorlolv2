use super::*;

impl Generator for Amumu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [(0, _1) /* Description 1 */, (1, _2) /* Innate */],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(Key::W, [(0, _1) /* Magic Damage Per Tick */])
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Physical Damage Reduction */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
