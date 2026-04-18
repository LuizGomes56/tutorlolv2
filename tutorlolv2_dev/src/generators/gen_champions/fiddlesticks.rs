use super::*;

impl Generator for Fiddlesticks {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 1, _1), (0, 2, _2), (2, 0, _3), (2, 1, _4)])
            .ability(Key::W, [(4, 0, _1), (4, 1, _2), (4, 2, _3), (4, 3, _4)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
