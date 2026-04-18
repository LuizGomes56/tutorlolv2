use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(2, 0, _1), (2, 1, _2)])
            .ability(Key::E, [(0, 1, _1)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4)])
            .end()
    }
}
