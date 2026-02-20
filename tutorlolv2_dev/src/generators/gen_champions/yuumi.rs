use super::*;

impl Generator<Champion> for Yuumi {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (1, 1, _2), (2, 0, _3)]);
        self.ability(Key::R, [(4, 0, _1), (4, 1, _2), (4, 2, _3)]);
        self.end()
    }
}
