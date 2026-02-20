use super::*;

impl Generator<Champion> for Elise {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)]);
        self.ability(Key::W, [(1, 0, _1)]);
        self.end()
    }
}
