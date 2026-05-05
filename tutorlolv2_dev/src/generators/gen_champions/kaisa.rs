use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Innate - Caustic Wounds */
                (1, _2), /* Innate - Caustic Wounds [1] */
                (2, _3), /* Innate - Caustic Wounds [2] */
                (3, _4), /* Innate - Caustic Wounds [3] */
                (4, _5), /* Innate - Caustic Wounds [4] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage Per Missile */
                (1, _2), /* Reduced Damage Per Missile */
                (2, _3), /* Total Evolved Single-Target Damage */
                (3, _4), /* Total Single-Target Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .end()
    }
}
