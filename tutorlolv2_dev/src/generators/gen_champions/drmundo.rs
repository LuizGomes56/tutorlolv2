use super::*;

impl Generator for DrMundo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Magic Damage */
                (2, _2), /* Minimum Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Magic Damage per Tick */
                (2, _3), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Attack Damage */
                (1, _2), /* Maximum Bonus Physical Damage */
                (4, _3), /* Minimum Bonus Physical Damage */
            ],
        )
        .end()
    }
}
