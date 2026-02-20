use super::*;

impl Generator<Champion> for Udyr {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (1, 1, _3)]);
        self.ability(Key::R, [(1, 0, _1), (1, 2, _2)]);
        self.end()
    }
}
