use super::*;

impl Generator for Briar {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1),     /* Innate */
                (3, _2),     /* Innate [10] */
                (4, _3),     /* Innate [11] */
                (5, _4),     /* Innate [12] */
                (6, _5),     /* Innate [13] */
                (7, _6),     /* Innate [14] */
                (8, _7),     /* Innate [15] */
                (9, _8),     /* Innate [16] */
                (10, _8Min), /* Innate [17] */
                (11, _8Min), /* Innate [18] */
                (12, _8Min), /* Innate [1] */
                (13, _8Min), /* Innate [2] */
                (14, _8Min), /* Innate [3] */
                (15, _8Min), /* Innate [4] */
                (16, _8Min), /* Innate [5] */
                (17, _8Min), /* Innate [6] */
                (18, _8Min), /* Innate [7] */
                (19, _8Max), /* Innate [8] */
                (20, _8Max), /* Innate [9] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Physical Damage */])
        .ability(Key::W, [(2, _1) /* Physical Damage */])
        .ability_nth(
            1,
            Key::W,
            [
                (0, _2), /* Bonus Physical Damage */
                (2, _3), /* Non-Champion Bonus Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Magic Damage */
                (3, _2), /* Maximum Magic Damage */
                (4, _3), /* Minimum Magic Damage */
                (5, _4), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(2, _1) /* Magic Damage */])
        .end()
    }
}
