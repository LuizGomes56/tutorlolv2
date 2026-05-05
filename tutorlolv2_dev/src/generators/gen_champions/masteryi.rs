use super::*;

impl Generator for MasterYi {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(3, _1) /* Innate */])
            .ability(
                Key::Q,
                [
                    (2, _1), /* Maximum Single-Target Damage */
                    (4, _2), /* Primary Physical Damage */
                    (5, _3), /* Reduced Damage per hit */
                ],
            )
            .ability(
                Key::W,
                [
                    (4, _1), /* Modified Damage Reduction */
                    (5, _2), /* Turret Modified Damage Reduction */
                ],
            )
            .ability(Key::E, [(0, _1) /* Bonus True Damage */])
            .end()
    }
}
