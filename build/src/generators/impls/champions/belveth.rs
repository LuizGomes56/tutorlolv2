use super::*;

impl Generator for Belveth {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (2, Max), /* Maximum Physical Damage per hit */
                    (4, Min), /* Minimum Physical Damage per hit */
                ],
            )
            .ability(
                Key::R,
                [
                    (2, _1),   /* Bonus True Damage */
                    (6, Void), /* True Damage */
                ],
            )
            .end()
    }
}
