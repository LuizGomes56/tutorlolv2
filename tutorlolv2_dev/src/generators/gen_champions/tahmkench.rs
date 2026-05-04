use super::*;

impl Generator for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */])
            .ability(Key::W, [(1, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Damage Stored into Grey Health */
                    (1, _2), /* Increased Damage Stored into Grey Health */
                ],
            )
            .ability_nth(1, Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
