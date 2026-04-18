use super::*;

impl Generator for Morgana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4)])
            .ability(Key::R, [(0, 1, _1), (0, 2, _2)])
            .end()
    }
}
