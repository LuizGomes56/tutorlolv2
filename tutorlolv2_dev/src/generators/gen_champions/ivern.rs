use super::*;

impl Generator for Ivern {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Ally Bonus Magic Damage */
                    (1, _2), /* Bonus Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .end()
    }
}
