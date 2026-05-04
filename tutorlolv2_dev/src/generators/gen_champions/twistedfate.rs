use super::*;

impl Generator for TwistedFate {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability_nth(1, Key::W, [(0, _2) /* Magic Damage */])
            .ability_nth(2, Key::W, [(0, _3) /* Magic Damage */])
            .ability_nth(3, Key::W, [(0, _4) /* Magic Damage */])
            .ability(Key::E, [(1, _1) /* Bonus Magic Damage */])
            .end()
    }
}
