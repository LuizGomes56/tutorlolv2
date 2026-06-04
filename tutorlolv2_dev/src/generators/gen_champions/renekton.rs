use super::*;

impl Generator for Renekton {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, Max), /* Enhanced Damage */
                (7, Min), /* Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Min), /* Physical Damage Per Hit */
                (1, Max), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (1, _1Min), /* Enhanced Physical Damage */
                (2, Min),   /* Physical Damage */
                (3, _1Max), /* Total Enhanced Damage */
                (4, Max),   /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, Min), /* Magic Damage Per Tick */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
