use super::*;

impl Generator for Olaf {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Physical Damage */])
            .ability(Key::E, [(0, _1) /* True Damage */])
            .ability(Key::R, [(0, _1) /* Bonus Attack Damage */])
            .end()
    }
}
