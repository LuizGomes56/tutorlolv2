use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 0, _2), (1, 0, _3)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)])
            .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
