use super::*;

impl Generator<Champion> for Fiddlesticks {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, _1), (0, 2, _2), (2, 0, _3), (2, 1, _4)]);
        self.ability(Key::W, [(4, 0, _1), (4, 1, _2), (4, 2, _3), (4, 3, _4)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1), (0, 1, _2)]);
        self.end()
    }
}
