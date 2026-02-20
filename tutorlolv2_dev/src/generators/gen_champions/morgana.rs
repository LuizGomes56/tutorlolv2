use super::*;

impl Generator<Champion> for Morgana {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4)]);
        self.ability(Key::R, [(0, 1, _1), (0, 2, _2)]);
        self.end()
    }
}
