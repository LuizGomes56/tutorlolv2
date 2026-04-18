use super::*;

impl Generator for Brand {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::E, [(1, 0, _1)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2)])
            .end()
    }
}
