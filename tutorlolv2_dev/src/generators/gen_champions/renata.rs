use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .end()
    }
}
