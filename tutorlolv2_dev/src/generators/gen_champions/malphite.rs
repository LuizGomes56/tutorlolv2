use super::*;

impl Generator for Malphite {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Additional Physical Damage */
                    (3, _2), /* Physical Damage */
                ],
            )
            .ability(Key::E, [(1, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
