use super::*;

impl Generator for TwistedFate {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(1, Void) /* Bonus Magic Damage */])
            .end()
    }
}
