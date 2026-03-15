use super::*;

impl Generator<Champion> for Jinx {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(1, 0, _1), (1, 1, _2), (2, 0, _3), (2, 1, _4)]);
        self.end()
    }
}
