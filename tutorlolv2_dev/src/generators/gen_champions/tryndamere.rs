use super::*;

impl Generator for Tryndamere {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Attack Damage per 1% missing health */
                (2, _2), /* Maximum Bonus Attack Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Attack Damage Reduction */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .end()
    }
}
