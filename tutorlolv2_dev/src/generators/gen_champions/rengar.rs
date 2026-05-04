use super::*;

impl Generator for Rengar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Additional Physical Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Physical Damage */])
            .end()
    }
}
