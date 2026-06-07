use super::*;

impl Generator for Vi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Physical Damage */
                (1, Min), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::W, [(1, Void) /* Bonus Physical Damage */])
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
