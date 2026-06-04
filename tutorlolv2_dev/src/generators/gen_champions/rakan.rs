use super::*;

impl Generator for Rakan {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .end()
    }
}
