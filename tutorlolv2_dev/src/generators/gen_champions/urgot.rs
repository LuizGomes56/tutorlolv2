use super::*;

impl Generator for Urgot {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::W, [(0, _1) /* Modified Physical Damage */])
            .ability(Key::E, [(0, _1) /* Physical Damage */])
            .ability(Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
