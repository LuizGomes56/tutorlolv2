use super::*;

impl Generator for Irelia {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Maximum Physical Damage */
                    (1, _2), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
