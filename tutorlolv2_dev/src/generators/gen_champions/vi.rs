use super::*;

impl Generator for Vi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Bonus Physical Damage */])
        .ability_nth(1, Key::W, [(1, _2) /* Bonus Physical Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability_nth(1, Key::E, [(0, _2) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
