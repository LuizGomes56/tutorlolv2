use super::*;

impl Generator<Champion> for Yasuo {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1), (2, 0, _2), (2, 1, _3), (2, 2, _4)]);
        self.ability(Key::R, [(3, 0, _1)]);
        self.end()
    }
}
