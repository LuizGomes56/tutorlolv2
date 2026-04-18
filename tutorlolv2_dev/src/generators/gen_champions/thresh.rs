use super::*;

impl Generator for Thresh {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
