use super::*;

impl Generator<Champion> for Zaahen {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 2, _1), (0, 3, _2), (3, 0, _3)]);
        self.ability(Key::W, [(0, 0, _1), (0, 1, _2)]);
        self.ability(Key::E, [(0, 0, _1), (2, 0, _2), (2, 1, _3)]);
        self.ability(Key::R, [(0, 1, _1)]);
        self.end()
    }
}
