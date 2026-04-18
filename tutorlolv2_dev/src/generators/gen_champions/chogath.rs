use super::*;

impl Generator for Chogath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (0, 1, _2), (0, 3, _3), (0, 4, _4)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
