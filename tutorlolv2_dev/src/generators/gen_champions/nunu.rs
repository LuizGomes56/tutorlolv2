use super::*;

impl Generator for Nunu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, _1), /* Champion Magic Damage */
                (5, _2), /* Non-Champion True Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Maximum Magic Damage */
                (2, _2), /* Minimum Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Magic Damage Per Hit */
                (2, _3), /* Maximum Total Magic Damage */
                (4, _4), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Maximum Magic Damage */])
        .end()
    }
}
