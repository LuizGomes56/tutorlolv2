use super::*;

impl Generator for XinZhao {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (2, _2), /* Innate [1] */
                (3, _3), /* Innate [2] */
                (4, _4), /* Innate [3] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Physical Damage */
                (1, _2), /* Total Bonus Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Physical Damage per Slash */
                (1, _2), /* Slash Total Physical Damage */
                (2, _3), /* Thrust Physical Damage */
                (3, _4), /* Total Physical Damage */
            ],
        )
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
