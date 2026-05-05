use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Medarda Maxim */
                (2, _2), /* Medarda Maxim [1] */
                (3, _3), /* Medarda Maxim [2] */
                (4, _4), /* Medarda Maxim [3] */
                (5, _5), /* Medarda Maxim [4] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Increased Physical Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Increased Physical Damage */
                (1, _4), /* Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Increased Physical Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Total Physical Damage */
            ],
        )
        .ability(Key::R, [(4, _1) /* Physical Damage */])
        .end()
    }
}
