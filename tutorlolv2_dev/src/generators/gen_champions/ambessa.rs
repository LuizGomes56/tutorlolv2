use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)])
            .ability(Key::W, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
