use super::*;

impl Generator for JarvanIV {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .end()
    }
}
