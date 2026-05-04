use super::*;

impl Generator for Quinn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Physical Damage */])
            .ability(Key::E, [(0, _1) /* Physical damage */])
            .ability_nth(1, Key::R, [(0, _1) /* Physical Damage */])
            .end()
    }
}
