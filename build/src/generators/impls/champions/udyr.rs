use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min),  /* Bonus Physical Damage */
                (2, Void), /* Bonus Physical Damage On-Hit */
                (3, Max),  /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, Min), /* Magic Damage per Tick */
                (3, Max), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
