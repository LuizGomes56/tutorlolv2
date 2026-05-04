use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Magic Damage */
                (1, _2), /* Minimum Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Maximum Magic Damage */
                (1, _4), /* Minimum Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (2, _1), /* Magic Damage Per Bolt */
                (3, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Damage Cap */
                (2, _2), /* Magic Damage */
                (3, _3), /* Maximum Mixed Damage */
            ],
        )
        .end()
    }
}
