use super::*;

impl Generator for Rakan {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(1, _1) /* Magic Damage */])
            .end()
    }
}
