use super::*;

impl Generator for Nocturne {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Attack Damage */
                (2, _2), /* Physical damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, _1), /* Magic Damage per Tick */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
