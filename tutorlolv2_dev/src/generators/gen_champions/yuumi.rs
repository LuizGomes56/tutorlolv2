use super::*;

impl Generator for Yuumi {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 1, _2), (2, 0, _3)])
            .ability(Key::R, [(4, 0, _1), (4, 1, _2), (4, 2, _3)])
            .end()
    }
}
