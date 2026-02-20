use super::*;

impl Generator<Champion> for Ziggs {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)]);
        self.ability(Key::W, [(1, 0, _1)]);
        self.ability(Key::E, [(1, 0, _1), (1, 1, _2), (1, 2, _3)]);
        self.ability(Key::R, [(1, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
