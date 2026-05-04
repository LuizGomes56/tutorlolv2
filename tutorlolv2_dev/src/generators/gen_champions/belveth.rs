use super::*;

impl Generator for Belveth {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, _1) /* Physical Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Damage Reduction */
                    (2, _2), /* Maximum Physical Damage per hit */
                    (4, _3), /* Minimum Physical Damage per hit */
                ],
            )
            .ability(
                Key::R,
                [
                    (2, _1), /* Bonus True Damage */
                    (6, _2), /* True Damage */
                ],
            )
            .end()
    }
}
