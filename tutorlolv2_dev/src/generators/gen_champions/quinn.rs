use super::*;

impl Generator for Quinn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
