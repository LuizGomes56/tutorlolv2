use super::*;

impl Generator for LeeSin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 0, _2), (0, 1, _3)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (1, 0, _2)])
            .end()
    }
}
