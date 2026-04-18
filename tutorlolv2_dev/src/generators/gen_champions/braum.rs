use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(0, 1, _1)])
            .ability(Key::R, [(1, 0, _1)])
            .end()
    }
}
