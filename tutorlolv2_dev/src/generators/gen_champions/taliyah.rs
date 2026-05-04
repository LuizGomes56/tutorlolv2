use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, _1), /* Empowered Damage */
                (3, _2), /* Magic Damage */
                (4, _3), /* Reduced Damage */
                (5, _4), /* Secondary Target Damage */
                (7, _5), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Detonation Magic Damage */
                (1, _2), /* Magic Damage */
                (2, _3), /* Total Maximum Detonation Damage */
            ],
        )
        .end()
    }
}
