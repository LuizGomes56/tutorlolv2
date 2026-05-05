use super::*;

impl Generator for Seraphine {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Description 2 */])
            .ability(
                Key::Q,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Maximum Enhanced Damage */
                ],
            )
            .ability(Key::E, [(1, _1) /* Magic Damage */])
            .ability(Key::R, [(1, _1) /* Magic Damage */])
            .end()
    }
}
