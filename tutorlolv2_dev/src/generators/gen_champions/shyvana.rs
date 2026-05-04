use super::*;

impl Generator for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Area Physical Damage */
                (1, _2), /* True Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Increased/Explosion Magic Damage */
                (1, _2), /* Magic Damage */
                (2, _3), /* Subsequent Explosion Damage */
            ],
        )
        .ability(Key::R, [(4, _1) /* Magic Damage */])
        .end()
    }
}
