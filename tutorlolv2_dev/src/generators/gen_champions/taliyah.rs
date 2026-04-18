use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 2, _3), (3, 0, _4)])
            .ability(Key::E, [(0, 0, _1), (1, 0, _2), (2, 0, _3)])
            .end()
    }
}
