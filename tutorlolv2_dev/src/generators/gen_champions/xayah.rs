use super::*;

impl Generator for Xayah {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (1, 0, _3), (1, 1, _4)])
            .ability(Key::E, [(2, 0, _1), (2, 1, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
