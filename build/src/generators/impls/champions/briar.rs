use super::*;

impl Generator for Briar {
    #[warn(unstable_features)]
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
                (10, _1Min), /* Innate [17] */
                (11, _2Min), /* Innate [18] */
                (12, _3Min), /* Innate [1] */
                (13, _4Min), /* Innate [2] */
                (14, _5Min), /* Innate [3] */
                (15, _6Min), /* Innate [4] */
                (16, _7Min), /* Innate [5] */
                (17, _8Min), /* Innate [6] */
                (18, _1Max), /* Innate [7] */
                (19, _2Max), /* Innate [8] */
                (20, _3Max), /* Innate [9] */
            ],
        )
        .ability(Key::Q, [(0, Void) /* Physical Damage */])
        .ability(Key::W, [(2, Void) /* Physical Damage */])
        .ability_nth(1, Key::W, [(0, Void) /* Bonus Physical Damage */])
        .ability(
            Key::E,
            [
                (0, _2),    /* Bonus Magic Damage */
                (3, Max),   /* Maximum Magic Damage */
                (4, Min),   /* Minimum Magic Damage */
                (5, _1Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(2, Void) /* Magic Damage */])
        .end()
    }
}
