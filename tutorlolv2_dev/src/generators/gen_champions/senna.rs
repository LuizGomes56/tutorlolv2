use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Physical Damage */])
            .ability(Key::W, [(0, _1) /* Physical Damage */])
            .ability(Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
