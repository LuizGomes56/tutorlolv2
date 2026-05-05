use super::*;

impl Generator for Mordekaiser {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1),    /* Darkness Rise */
                (1, _2),    /* Darkness Rise [1] */
                (2, _3),    /* Darkness Rise [2] */
                (3, _4),    /* Darkness Rise [3] */
                (4, _5),    /* Darkness Rise [4] */
                (5, _6),    /* Darkness Rise [5] */
                (6, _7),    /* Darkness Rise [6] */
                (7, _8),    /* Darkness Rise [7] */
                (8, _8Min), /* Darkness Rise [8] */
                (9, _8Min), /* Innate */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Isolated Damage Increase */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
