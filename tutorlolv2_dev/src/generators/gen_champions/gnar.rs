use super::*;

impl Generator<Champion> for Gnar {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3)]);
        self.ability(Key::W, [(2, 0, _1), (0, 0, _2)]);
        self.ability(Key::E, [(3, 0, _1), (0, 0, _2)]);
        self.ability(Key::R, [(0, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
