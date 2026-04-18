use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(Key::W, [(0, 1, _1), (0, 2, _2), (0, 3, _3), (0, 4, _4)])
            .end()
    }
}
