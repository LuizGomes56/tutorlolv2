use super::*;

impl Generator for Yone {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 2, _3)])
            .ability(Key::E, [(3, 0, _1)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2), (1, 2, _3)])
            .end()
    }
}
