use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),   /* Bonus Magic Damage */
                (1, _1Min), /* Increased Bonus Damage */
                (4, _1Max), /* Total Increased Damage */
                (5, Max),   /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .end()
    }
}
