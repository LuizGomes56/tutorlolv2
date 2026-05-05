use super::*;

impl Generator for Darius {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1),     /* Hemorrhage */
                (2, _2),     /* Hemorrhage [1] */
                (3, _3),     /* Hemorrhage [2] */
                (4, _4),     /* Hemorrhage [3] */
                (5, _5),     /* Hemorrhage [4] */
                (6, _6),     /* Hemorrhage [5] */
                (7, _7),     /* Hemorrhage [6] */
                (8, _8),     /* Hemorrhage [7] */
                (9, _8Min),  /* Hemorrhage [8] */
                (11, _8Min), /* Noxian Might */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage (Blade) */
                (1, _2), /* Reduced Damage (Handle) */
            ],
        )
        .ability(Key::W, [(0, _1) /* Bonus Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Bonus Damage Per Stack */
                (1, _2), /* Maximum True Damage */
                (2, _3), /* True Damage */
            ],
        )
        .end()
    }
}
