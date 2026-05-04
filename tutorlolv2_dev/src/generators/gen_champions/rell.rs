use super::*;

impl Generator for Rell {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(1, _1) /* Magic Damage */])
            .ability_nth(1, Key::W, [(0, _2) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, _1) /* Bonus Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage Per Tick */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
