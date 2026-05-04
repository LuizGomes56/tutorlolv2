use super::*;

impl Generator for Veigar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
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
