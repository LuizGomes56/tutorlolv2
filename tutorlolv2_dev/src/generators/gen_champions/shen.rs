use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Increased Bonus Damage */
                (4, _3), /* Total Increased Damage */
                (5, _4), /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .end()
    }
}
