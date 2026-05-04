use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        self.ability(
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
