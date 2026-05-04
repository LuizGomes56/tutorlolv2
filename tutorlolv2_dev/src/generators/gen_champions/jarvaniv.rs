use super::*;

impl Generator for JarvanIV {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Physical Damage */])
            .ability(Key::E, [(1, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
