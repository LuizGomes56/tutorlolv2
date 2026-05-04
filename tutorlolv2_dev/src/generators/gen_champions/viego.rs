use super::*;

impl Generator for Viego {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Physical Damage */
                (1, _2), /* Minimum Bonus Damage */
                (2, _3), /* Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
