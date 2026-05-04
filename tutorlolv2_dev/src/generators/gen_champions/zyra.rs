use super::*;

impl Generator for Zyra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
