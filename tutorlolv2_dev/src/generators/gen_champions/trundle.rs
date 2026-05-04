use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Attack Damage Reduction */
                (1, _2), /* Bonus Attack Damage */
                (2, _3), /* Bonus Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (2, _1), /* Initial Magic Damage */
                (3, _2), /* Magic Damage Per Second */
                (5, _3), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
