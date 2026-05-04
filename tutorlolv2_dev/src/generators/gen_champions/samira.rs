use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Physical Damage per Hit */
                    (1, _2), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(1, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, _1), /* Physical Damage Per Shot */
                    (3, _2), /* Total Physical Damage */
                ],
            )
            .end()
    }
}
