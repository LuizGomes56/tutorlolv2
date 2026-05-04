use super::*;

impl Generator for KogMaw {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */])
            .ability(Key::W, [(1, _1) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Maximum Magic Damage */
                    (1, _2), /* Minimum Magic Damage */
                ],
            )
            .end()
    }
}
