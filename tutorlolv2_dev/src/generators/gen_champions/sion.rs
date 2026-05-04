use super::*;

impl Generator for Sion {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Base Damage Increase */
                (3, _2), /* Maximum Physical Damage */
                (6, _3), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability_nth(1, Key::W, [(0, _2) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .ability_nth(
            1,
            Key::R,
            [
                (0, _3), /* Maximum Physical Damage */
                (1, _4), /* Minimum Physical Damage */
            ],
        )
        .end()
    }
}
