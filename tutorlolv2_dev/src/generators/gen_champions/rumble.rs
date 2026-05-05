use super::*;

impl Generator for Rumble {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Overheated */
                (3, _2), /* Overheated [1] */
                (4, _3), /* Overheated [2] */
                (5, _4), /* Overheated [3] */
                (6, _5), /* Overheated [4] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1),  /* Enhanced Damage per Second */
                (1, _2),  /* Enhanced Damage per Tick */
                (4, _3),  /* Magic Damage per Second */
                (5, _4),  /* Magic Damage per Tick */
                (6, _5),  /* Maximum Enhanced Damage */
                (8, _6),  /* Maximum Magic Damage */
                (10, _7), /* Minimum Enhanced Damage */
                (12, _8), /* Minimum Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1),  /* Enhanced Damage */
                (3, _2),  /* Magic Damage */
                (6, _3),  /* Total Enhanced Damage */
                (10, _4), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage per Second */
                (1, _2), /* Magic Damage per Tick */
                (2, _3), /* Maximum Magic Damage */
            ],
        )
        .end()
    }
}
