use super::*;

impl Generator for MonkeyKing {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (1, _2), /* Innate [1] */
                (2, _3), /* Innate [2] */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Bonus Physical Damage */])
        .ability(Key::W, [(0, _1) /* Clone Outgoing Damage */])
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Maximum Total Physical Damage */
                (1, _2), /* Physical Damage Per Tick */
                (2, _3), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
