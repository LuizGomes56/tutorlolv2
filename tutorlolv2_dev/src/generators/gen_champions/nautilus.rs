use super::*;

impl Generator<Champion> for Nautilus {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)]);
        self.ability(Key::W, [(1, 0, _1), (1, 1, _2)]);
        self.ability(Key::E, [(0, 0, _1), (0, 1, _2), (0, 2, _3), (1, 0, _4)]);
        self.ability(Key::R, [(0, 0, _1), (1, 0, _2)]);
        self.end()
    }
}
