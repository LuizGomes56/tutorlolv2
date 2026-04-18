use super::*;

impl Generator for MonkeyKing {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 1, _1)])
            .ability(Key::W, [(2, 0, _1)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2), (4, 0, _3)])
            .end()
    }
}
