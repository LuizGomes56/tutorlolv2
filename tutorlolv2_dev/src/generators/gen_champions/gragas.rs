use super::*;

impl Generator for Gragas {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Magic Damage */
                (3, _2), /* Minimum Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::Q,
            [
                (0, _3), /* Maximum Magic Damage */
                (3, _4), /* Minimum Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Damage Reduction */
            ],
        )
        .ability_nth(
            1,
            Key::W,
            [
                (0, _3), /* Bonus Magic Damage */
                (1, _4), /* Damage Reduction */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
