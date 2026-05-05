use super::*;

impl Generator for Irelia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Ionian Fervor */
                (3, _2), /* Ionian Fervor [1] */
                (4, _3), /* Ionian Fervor [2] */
                (5, _4), /* Ionian Fervor [3] */
                (6, _5), /* Ionian Fervor [4] */
                (7, _6), /* Ionian Fervor [5] */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Physical Damage */])
        .ability(
            Key::W,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
