use super::*;

impl Generator for Pantheon {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Hurl Physical Damage */
                (1, _2), /* Hurl Secondary Physical Damage */
                (2, _3), /* Increased Hurl Damage */
                (3, _4), /* Increased Hurl Secondary Damage */
                (4, _5), /* Increased Thrust Damage */
                (5, _6), /* Thrust Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Physical Damage */])
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (1, _1), /* Magic Damage */
                (2, _2), /* Reduced Damage */
            ],
        )
        .end()
    }
}
