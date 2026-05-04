use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Bonus Physical Damage per Hit */
                (3, _2), /* Total Bonus Physical Damage */
            ],
        )
        .ability_nth(1, Key::Q, [(1, _3) /* Physical Damage */])
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
