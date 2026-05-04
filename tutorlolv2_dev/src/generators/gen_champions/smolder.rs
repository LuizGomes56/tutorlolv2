use super::*;

impl Generator for Smolder {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Explosion Physical Damage */
                    (1, _2), /* Glob Physical Damage */
                    (2, _3), /* Total Physical Damage On Champion Hit */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Increased Physical Damage */
                    (1, _2), /* Physical Damage */
                ],
            )
            .end()
    }
}
