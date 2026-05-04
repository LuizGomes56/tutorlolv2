use super::*;

impl Generator for Kled {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Physical Damage */
                (3, _2), /* Total Physical Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Maximum Damage */
                (1, _4), /* Physical Damage */
                (2, _5), /* Reduced Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Additional Physical Damage */
                (1, _2), /* Structure Additional Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, _1), /* Maximum Magic Damage */
                (2, _2), /* Minimum Magic Damage */
            ],
        )
        .end()
    }
}
