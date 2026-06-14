use super::*;

impl Generator for Rengar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Additional Physical Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .end()
    }
}
