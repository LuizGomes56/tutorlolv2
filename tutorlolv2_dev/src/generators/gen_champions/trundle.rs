use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(Key::R, [(0, 1, _1), (1, 2, _2), (1, 3, _3)])
            .end()
    }
}
