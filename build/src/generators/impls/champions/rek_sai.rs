use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Bonus Physical Damage */
                (1, Max), /* Total Bonus Physical Damage */
            ],
        )
        .ability_nth(1, Key::Q, [(0, Void) /* Magic Damage */])
        .ability_nth(1, Key::W, [(0, Void) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, Void), /* Physical Damage */
                (1, _1),   /* True Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
