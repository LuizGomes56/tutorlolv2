use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
