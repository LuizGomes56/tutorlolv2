use super::*;

impl Generator for Lux {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(2, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
