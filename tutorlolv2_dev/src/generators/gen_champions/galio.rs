use super::*;

impl Generator<Champion> for Galio {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1), (0, 1, _2), (3, 0, _3), (3, 1, _4)]);
        self.ability(Key::E, [(1, 0, _1), (1, 1, _2)]);
        self.ability(Key::R, [(1, 0, _1)]);
        self.end()
    }
}
