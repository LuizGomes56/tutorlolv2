use super::*;

impl Generator for Nidalee {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Magic Damage */
                (1, _2), /* Minimum Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Increased Damage Modifier */
                (1, _4), /* Maximum Increased Damage */
                (2, _5), /* Maximum Magic Damage */
                (3, _6), /* Minimum Magic Damage */
                (4, _7), /* Prowl-Enhanced Maximum Damage */
                (5, _8), /* Prowl-Enhanced Minimum Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage Per Tick */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability_nth(1, Key::W, [(0, _3) /* Magic Damage */])
        .ability_nth(1, Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
