use super::*;

impl Generator for Yuumi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Increased Damage */
                (2, _2), /* Magic Damage */
                (3, _3), /* Maximum Bonus Magic Damage On-Hit */
                (4, _4), /* Minimum Bonus Magic Damage On-Hit */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Magic Damage per Hit */
                (2, _2), /* Reduced Damage per Hit */
                (4, _3), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
