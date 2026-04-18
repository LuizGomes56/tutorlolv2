use super::*;

impl Generator for Jhin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::E, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2), (2, 0, _3), (2, 1, _4)])
            .end()
    }
}
