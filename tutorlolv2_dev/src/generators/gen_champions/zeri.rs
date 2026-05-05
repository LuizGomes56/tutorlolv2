use super::*;

impl Generator for Zeri {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Basic Attack */
                (1, _2), /* Basic Attack [1] */
                (2, _3), /* Description 2 */
                (3, _4), /* Description 2 [1] */
                (4, _5), /* Description 2 [2] */
                (5, _6), /* Description 2 [3] */
                (6, _7), /* Innate */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1), /* Infinity Edge Damage per Hit */
                (2, _2), /* Physical Damage per Hit */
                (3, _3), /* Total Critical Strike Damage */
                (4, _4), /* Total Infinity Edge Damage */
                (5, _5), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Critical Strike Damage */
                (1, _2), /* Infinity Edge Damage */
                (2, _3), /* Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Burst Fire Bonus Magic Damage */
                (1, _2), /* Burst Fire Secondary Target Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
