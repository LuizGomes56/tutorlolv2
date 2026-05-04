use super::*;

impl Generator for Ryze {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */])
            .ability_nth(1, Key::Q, [(1, _2) /* Magic Damage */])
            .ability_nth(2, Key::Q, [(1, _3) /* Magic Damage */])
            .ability_nth(3, Key::Q, [(1, _4) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Bonus Overload Damage */])
            .end()
    }
}
