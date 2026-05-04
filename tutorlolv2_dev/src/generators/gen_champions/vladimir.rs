use super::*;

impl Generator for Vladimir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Increased Damage */
                (2, _2), /* Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (1, _3), /* Increased Damage */
                (2, _4), /* Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage Per Tick */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Maximum Magic Damage */
                (1, _2), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::R, [(1, _1) /* Magic damage */])
        .end()
    }
}
