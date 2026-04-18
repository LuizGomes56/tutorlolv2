use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::W, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::E, [(1, 0, _1), (2, 0, _2), (2, 1, _3)])
            .end()
    }
}
