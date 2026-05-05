use super::*;

impl Generator for Lillia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (4, _1),  /* Innate */
                (5, _2),  /* Innate [1] */
                (6, _3),  /* Innate [2] */
                (7, _4),  /* Innate [3] */
                (8, _5),  /* Innate [4] */
                (9, _6),  /* Innate [5] */
                (10, _7), /* Innate [6] */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1), /* Magic Damage */
                (3, _2), /* Total Mixed Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Increased Damage */
                (2, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
