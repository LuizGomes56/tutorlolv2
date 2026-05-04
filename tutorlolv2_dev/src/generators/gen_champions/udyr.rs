use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Bonus Physical Damage */
                (2, _2), /* Bonus Physical Damage On-Hit */
                (3, _3), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Magic Damage per Tick */
                (3, _2), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
