use super::*;

impl Generator for Aurora {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Maximum Magic Damage */
                (2, _3), /* Minimum Magic Damage */
                (3, _4), /* Subsequent Bolt Maximum Magic Damage */
                (4, _5), /* Subsequent Bolt Minimum Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
