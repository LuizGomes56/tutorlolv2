use super::*;

impl Generator for Zaahen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 2, _1), (0, 3, _2), (3, 0, _3)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::E, [(0, 0, _1), (2, 0, _2), (2, 1, _3)])
            .ability(Key::R, [(0, 1, _1)])
            .end()
    }
}
