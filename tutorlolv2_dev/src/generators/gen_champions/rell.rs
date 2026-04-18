use super::*;

impl Generator for Rell {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (0, 0, _2)])
            .ability(Key::E, [(1, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
