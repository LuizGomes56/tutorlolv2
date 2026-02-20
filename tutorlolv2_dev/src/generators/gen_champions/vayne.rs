use super::*;

impl Generator<Champion> for Vayne {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(1, 0, _1), (2, 0, _2), (2, 1, _3)]);
        self.ability(Key::E, [(0, 0, _1), (1, 0, _2), (1, 1, _3)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
