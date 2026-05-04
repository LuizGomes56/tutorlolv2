use super::*;

impl Generator for Illaoi {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Damage Increase */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Additional Physical Damage */
                    (1, _2), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Damage Transmission */])
            .ability_nth(1, Key::E, [(0, _2) /* Damage Transmission */])
            .ability(Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
