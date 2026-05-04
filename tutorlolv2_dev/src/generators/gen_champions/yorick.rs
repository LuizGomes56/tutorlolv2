use super::*;

impl Generator for Yorick {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(Key::E, [(3, _1) /* Magic Damage */])
            .ability_nth(1, Key::E, [(3, _2) /* Magic Damage */])
            .end()
    }
}
