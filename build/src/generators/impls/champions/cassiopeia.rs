use super::*;

impl Generator for Cassiopeia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Magic Damage Per Tick */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Min), /* Magic Damage Per Second */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Bonus Magic Damage */
                (3, Max), /* Total Enhanced Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .end()
    }
}
