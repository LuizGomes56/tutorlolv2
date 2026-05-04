use super::*;

impl Generator for Warwick {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Damage Reduction */])
            .ability(Key::R, [(0, _1) /* Total Magic Damage */])
            .end()
    }
}
