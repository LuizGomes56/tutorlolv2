use super::*;

impl Generator for Locke {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate */
                (1, _2), /* Innate [1] */
                (2, _3), /* Innate [2] */
                (3, _4), /* Innate [3] */
                (4, _5), /* Innate [4] */
                (5, _6), /* Innate [5] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage per Stack */
                (1, _2), /* Magic Damage per Nail */
                (2, _3), /* Maximum Nail Damage */
                (3, _4), /* Maximum Stack Bonus Damage */
                (4, _5), /* Maximum Total Magic Damage */
                (5, _6), /* Total Magic Damage per Nail */
            ],
        )
        .ability(Key::W, [(0, _1) /* Damage taken grey health cap */])
        .ability(
            Key::E,
            [
                (0, _1), /* Blink Magic Damage */
                (1, _2), /* Dash Magic Damage */
                (2, _3), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(1, _1) /* Magic Damage */])
        .end()
    }
}
