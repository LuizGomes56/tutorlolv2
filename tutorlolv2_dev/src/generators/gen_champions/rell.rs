use super::*;

impl Generator for Rell {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability_nth(1, Key::W, [(0, _1) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Bonus Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
