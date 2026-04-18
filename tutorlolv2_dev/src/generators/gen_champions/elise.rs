use super::*;

impl Generator for Elise {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)])
            .ability(Key::W, [(1, 0, _1)])
            .end()
    }
}
