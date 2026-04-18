use super::*;

impl Generator for Malzahar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(3, 0, _1), (3, 1, _2)])
            .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2), (1, 0, _3), (1, 1, _4)])
            .end()
    }
}
