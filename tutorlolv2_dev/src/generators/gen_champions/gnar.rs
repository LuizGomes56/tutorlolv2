use super::*;

impl Generator for Gnar {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Physical Damage */
                (1, Min), /* Reduced Damage */
            ],
        )
        .ability_nth(1, Key::Q, [(0, Mega) /* Physical Damage */])
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability_nth(1, Key::W, [(0, Mega) /* Physical Damage */])
        .ability(Key::E, [(1, Void) /* Physical Damage */])
        .ability_nth(1, Key::E, [(0, Mega) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (2, Max), /* Increased Damage */
                (3, Min), /* Physical Damage */
            ],
        )
        .end()
    }
}
