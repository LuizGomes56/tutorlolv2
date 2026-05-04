use super::*;

impl Generator for Varus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Maximum Reduced Damage */
                (2, _3), /* Minimum Physical Damage */
                (3, _4), /* Minimum Reduced Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Active Maximum Magic Damage */
                (1, _2), /* Active Minimum Magic Damage */
                (2, _3), /* Bonus Magic Damage */
                (3, _4), /* Bonus Magic Damage at Max Stacks */
                (4, _5), /* Bonus Magic Damage per Stack */
                (5, _6), /* Maximum Bonus Magic Damage at Max Stacks */
                (6, _7), /* Maximum Bonus Magic Damage per Stack */
            ],
        )
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
