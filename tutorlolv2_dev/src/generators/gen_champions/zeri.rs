use super::*;

impl Generator for Zeri {
    fn generate(&mut self) -> MayFail {
        self.ability(
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
