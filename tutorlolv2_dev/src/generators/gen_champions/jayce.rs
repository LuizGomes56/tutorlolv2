use super::*;

impl Generator<Champion> for Jayce {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 0, _2), (1, 0, _3)]);
        self.ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)]);
        self.ability(Key::E, [(0, 0, _1), (0, 1, _2)]);
        self.end()
    }
}
