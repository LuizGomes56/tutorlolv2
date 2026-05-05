use super::*;

impl Generator for Zaahen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Determination */
                (1, _2), /* Determination [1] */
                (2, _3), /* Determination [2] */
                (3, _4), /* Innate */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Physical Damage */
                (3, _2), /* Physical Damage per Hit */
                (4, _3), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Initial Physical Damage */
                (1, _2), /* Subsequent Physical Damage */
                (2, _3), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Increased Physical Damage */
                (2, _3), /* Physical Damage */
            ],
        )
        .ability(Key::R, [(2, _1) /* Physical Damage */])
        .end()
    }
}
