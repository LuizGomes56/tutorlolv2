use super::*;

impl Generator for Evelynn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Magic Damage */
                (2, _3), /* Maximum Magic Damage */
                (3, _4), /* Total Bonus Damage */
                (4, _5), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Bonus Magic Damage */])
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability_nth(1, Key::E, [(0, _2) /* Empowered Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Empowered Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .end()
    }
}
