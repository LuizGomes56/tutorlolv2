use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (3, _1), /* Innate - Rage Gene */
                (4, _2), /* Innate - Rage Gene [1] */
                (5, _3), /* Innate - Rage Gene [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Reduced Damage */
            ],
        )
        .ability_nth(1, Key::Q, [(0, _3) /* Physical Damage */])
        .ability(Key::W, [(0, _1) /* Bonus Magic Damage */])
        .ability_nth(1, Key::W, [(0, _2) /* Physical Damage */])
        .ability(Key::E, [(1, _1) /* Physical Damage */])
        .ability_nth(1, Key::E, [(0, _2) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (2, _1), /* Increased Damage */
                (3, _2), /* Physical Damage */
            ],
        )
        .end()
    }
}
