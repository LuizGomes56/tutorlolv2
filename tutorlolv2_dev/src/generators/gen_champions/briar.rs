use super::*;

impl Generator for Briar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(2, 2, _1), (0, 0, _2), (1, 0, _3)])
            .ability(Key::E, [(2, 0, _1), (2, 1, _2), (3, 0, _3), (3, 1, _4)])
            .ability(Key::R, [(3, 0, _1)])
            .end()
    }
}
