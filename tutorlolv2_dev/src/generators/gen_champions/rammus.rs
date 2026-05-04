use super::*;

impl Generator for Rammus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
