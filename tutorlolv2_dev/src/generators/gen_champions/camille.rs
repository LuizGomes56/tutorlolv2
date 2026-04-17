use super::*;

impl Generator<Champion> for Camille {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, _1), (3, 0, _2)])
            .ability(Key::W, [(0, 0, _1), (1, 0, _2), (2, 0, _3), (2, 1, _4)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(2, 0, _1)]);
        self.end()
    }
}
