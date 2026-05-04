use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(1, _1) /* Damage reduction */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
