use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::E, [(1, 0, _1), (1, 1, _2), (1, 2, _3)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2)])
            .end()
    }
}
