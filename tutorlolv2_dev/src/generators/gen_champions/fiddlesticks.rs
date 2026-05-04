use super::*;

impl Generator for Fiddlesticks {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, _1), /* Increased Magic Damage */
                (2, _2), /* Increased Minimum Damage */
                (3, _3), /* Magic Damage */
                (4, _4), /* Minimum Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (1, _1), /* Damage per Instance */
                (2, _2), /* Damage per second */
                (3, _3), /* Last Tick of Damage */
                (7, _4), /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage per Tick */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
