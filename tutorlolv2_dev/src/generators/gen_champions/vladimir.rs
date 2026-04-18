use super::*;

impl Generator for Vladimir {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 1, _1), (2, 0, _2)])
            .ability(Key::W, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::E, [(4, 0, _1), (4, 1, _2)])
            .ability(Key::R, [(1, 1, _1)])
            .end()
    }
}
