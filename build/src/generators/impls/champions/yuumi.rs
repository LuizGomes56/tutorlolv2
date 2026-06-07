use super::*;

impl Generator for Yuumi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Max),   /* Increased Damage */
                (2, Min),   /* Magic Damage */
                (3, _1Max), /* Maximum Bonus Magic Damage On-Hit */
                (4, _1Min), /* Minimum Bonus Magic Damage On-Hit */
            ],
        )
        .ability(
            Key::R,
            [
                (1, Void), /* Magic Damage per Hit */
                (2, Min),  /* Reduced Damage per Hit */
                (4, Max),  /* Total Magic Damage */
            ],
        )
        .end()
    }
}
